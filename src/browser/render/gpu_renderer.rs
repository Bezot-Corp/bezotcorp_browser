use std::num::NonZeroUsize;
use std::sync::Arc;

use vello::wgpu::{self, CurrentSurfaceTexture, TextureViewDescriptor};
use vello::{AaConfig, AaSupport, RenderParams, RendererOptions, Scene};
use winit::window::Window;

use crate::browser::chrome::BrowserTheme;
use crate::browser::render::{BlitPipeline, BrowserFrame, GpuRendererError, TextRenderer};
use crate::browser::ui::{Component, DocumentView, RenderContext, Toolbar};

pub(crate) struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    renderer: vello::Renderer,
    render_texture: wgpu::Texture,
    render_view: wgpu::TextureView,
    blit_pipeline: BlitPipeline,
    text: TextRenderer,
    theme: BrowserTheme,
}

impl GpuRenderer {
    pub(crate) async fn new(window: Arc<Window>) -> Result<Self, GpuRendererError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|_| GpuRendererError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("BCB GPU Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::default(),
            })
            .await?;

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .find(|format| format.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let renderer = vello::Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )?;

        let (render_texture, render_view) =
            Self::create_render_texture(&device, surface_config.width, surface_config.height);

        let blit_pipeline = BlitPipeline::new(&device, surface_format, &render_view);

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            renderer,
            render_texture,
            render_view,
            blit_pipeline,
            text: TextRenderer::new(),
            theme: BrowserTheme::default(),
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        let (render_texture, render_view) =
            Self::create_render_texture(&self.device, width, height);

        self.render_texture = render_texture;
        self.render_view = render_view;
        self.blit_pipeline
            .update_render_view(&self.device, &self.render_view);
    }

    pub(crate) fn render(&mut self, frame: BrowserFrame<'_>) -> Result<(), GpuRendererError> {
        let mut scene = Scene::new();

        {
            let mut cx = RenderContext::new(&mut scene, &mut self.text, &self.theme);

            DocumentView {
                render_tree: frame.render_tree,
            }
            .render(&mut cx);

            Toolbar {
                state: frame.toolbar_state,
                loading: frame.loading_state,
                window_width: self.surface_config.width as f64,
            }
            .render(&mut cx);
        }

        self.renderer.render_to_texture(
            &self.device,
            &self.queue,
            &scene,
            &self.render_view,
            &RenderParams {
                base_color: self.theme.background(),
                width: self.surface_config.width,
                height: self.surface_config.height,
                antialiasing_method: AaConfig::Area,
            },
        )?;

        self.present()
    }

    fn present(&mut self) -> Result<(), GpuRendererError> {
        let surface_texture = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture)
            | CurrentSurfaceTexture::Suboptimal(texture) => texture,
            _ => return Err(GpuRendererError::SurfaceTexture),
        };

        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("BCB present encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("BCB blit pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            self.blit_pipeline.draw(&mut pass);
        }

        self.queue.submit([encoder.finish()]);
        surface_texture.present();

        Ok(())
    }

    fn create_render_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("BCB vello render texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        (texture, view)
    }
}
