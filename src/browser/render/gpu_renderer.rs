use std::num::NonZeroUsize;
use std::sync::Arc;

use vello::kurbo::{Affine, BezPath, Line, Point, Rect, RoundedRect, Stroke};
use vello::peniko::{Color, Fill};
use vello::wgpu::{self, CurrentSurfaceTexture, TextureViewDescriptor};
use vello::{AaConfig, AaSupport, RenderParams, RendererOptions, Scene};
use winit::window::Window;

use crate::browser::render::{GpuRendererError, RenderCommand, RenderTree};
use crate::browser::state::{BrowserLoadingState, BrowserToolbarState};

// ── Chrome constants ──────────────────────────────────────────────────────────
const TOOLBAR_H: f64 = 72.0;
const ACCENT_H: f64 = 3.0;
const BTN_Y: f64 = 14.0;
const BTN_W: f64 = 32.0;
const BTN_H: f64 = 32.0;
const BTN_RADIUS: f64 = 6.0;
const BTN_BACK_X: f64 = 8.0;
const BTN_FWD_X: f64 = 48.0;
const BTN_RELOAD_X: f64 = 88.0;
const ADDR_X: f64 = 136.0;
const ADDR_Y: f64 = 14.0;
const ADDR_H: f64 = 32.0;
const ADDR_RADIUS: f64 = 8.0;
const ADDR_MARGIN_RIGHT: f64 = 16.0;

// ── Colors ────────────────────────────────────────────────────────────────────
const COL_TOOLBAR: Color = Color::from_rgba8(28, 28, 32, 255);
const COL_BTN_ON: Color = Color::from_rgba8(64, 64, 72, 255);
const COL_BTN_OFF: Color = Color::from_rgba8(38, 38, 44, 255);
const COL_ADDR: Color = Color::from_rgba8(44, 44, 52, 255);
const COL_ADDR_ACTIVE: Color = Color::from_rgba8(48, 56, 88, 255);
const COL_ICON_ON: Color = Color::from_rgba8(220, 220, 230, 255);
const COL_ICON_OFF: Color = Color::from_rgba8(90, 90, 100, 255);
const COL_ACCENT_LOAD: Color = Color::from_rgba8(0, 170, 210, 255);
const COL_ACCENT_IDLE: Color = Color::from_rgba8(60, 60, 70, 255);
const COL_ADDR_TEXT: Color = Color::from_rgba8(200, 200, 215, 255);
const COL_BG: Color = Color::from_rgba8(18, 18, 22, 255);

// Blit shader : triangle plein écran qui recopie la texture intermédiaire vers la surface
const BLIT_SHADER: &str = r#"
var<private> POS: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 3.0, -1.0),
    vec2<f32>(-1.0,  3.0),
);
var<private> UV: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 1.0),
    vec2<f32>(2.0, 1.0),
    vec2<f32>(0.0, -1.0),
);
struct Vout { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };
@vertex fn vs(@builtin(vertex_index) i: u32) -> Vout {
    return Vout(vec4<f32>(POS[i], 0.0, 1.0), UV[i]);
}
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@fragment fn fs(in: Vout) -> @location(0) vec4<f32> {
    return textureSample(tex, smp, in.uv);
}
"#;

// ── Renderer ──────────────────────────────────────────────────────────────────
pub(crate) struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    renderer: vello::Renderer,
    // texture intermédiaire Rgba8Unorm pour vello
    render_texture: wgpu::Texture,
    render_view: wgpu::TextureView,
    // blit vers la surface sRGB
    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group_layout: wgpu::BindGroupLayout,
    blit_bind_group: wgpu::BindGroup,
    blit_sampler: wgpu::Sampler,
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
            .find(|f| f.is_srgb())
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
            Self::create_render_texture(&device, size.width.max(1), size.height.max(1));

        let blit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("blit sampler"),
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let blit_bind_group_layout = Self::create_blit_bind_group_layout(&device);
        let blit_bind_group = Self::create_blit_bind_group(
            &device,
            &blit_bind_group_layout,
            &render_view,
            &blit_sampler,
        );
        let blit_pipeline =
            Self::create_blit_pipeline(&device, &blit_bind_group_layout, surface_format);

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            renderer,
            render_texture,
            render_view,
            blit_pipeline,
            blit_bind_group_layout,
            blit_bind_group,
            blit_sampler,
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
        let (tex, view) = Self::create_render_texture(&self.device, width, height);
        self.render_texture = tex;
        self.render_view = view;
        self.blit_bind_group = Self::create_blit_bind_group(
            &self.device,
            &self.blit_bind_group_layout,
            &self.render_view,
            &self.blit_sampler,
        );
    }

    // Remplace la méthode render() existante
    pub(crate) fn render(
        &mut self,
        render_tree: &RenderTree,
        toolbar_state: &BrowserToolbarState,
        loading_state: BrowserLoadingState,
    ) -> Result<(), GpuRendererError> {
        let width = self.surface_config.width as f64;
        let mut scene = Scene::new();
        Self::build_content(&mut scene, render_tree);
        Self::build_chrome(&mut scene, toolbar_state, loading_state, width);

        self.renderer.render_to_texture(
            &self.device,
            &self.queue,
            &scene,
            &self.render_view,
            &RenderParams {
                base_color: COL_BG,
                width: self.surface_config.width,
                height: self.surface_config.height,
                antialiasing_method: AaConfig::Area,
            },
        )?;

        let surface_texture = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(t) | CurrentSurfaceTexture::Suboptimal(t) => t,
            _ => return Err(GpuRendererError::SurfaceTexture),
        };
        let surface_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("blit encoder"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("blit pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    depth_slice: None, // wgpu 29
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None, // wgpu 29
            });
            pass.set_pipeline(&self.blit_pipeline);
            pass.set_bind_group(0, &self.blit_bind_group, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit([encoder.finish()]);
        surface_texture.present();
        Ok(())
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn create_render_texture(
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("vello render texture"),
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

    fn create_blit_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blit bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    fn create_blit_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blit bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    // Remplace la méthode create_blit_pipeline() existante
    fn create_blit_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        surface_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("blit shader"),
            source: wgpu::ShaderSource::Wgsl(BLIT_SHADER.into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blit pipeline layout"),
            bind_group_layouts: &[Some(layout)], // wgpu 29 : Option<&BindGroupLayout>
            immediate_size: 0,                   // wgpu 29 : remplace push_constant_ranges
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("blit pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None, // wgpu 29 : remplace multiview
            cache: None,
        })
    }

    // ── Content ───────────────────────────────────────────────────────────────

    fn build_content(scene: &mut Scene, render_tree: &RenderTree) {
        for command in &render_tree.commands {
            Self::push_command(scene, command);
        }
    }

    fn push_command(scene: &mut Scene, command: &RenderCommand) {
        match command {
            RenderCommand::Clear { r, g, b, a } => {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(*r, *g, *b, *a),
                    None,
                    &Rect::new(0.0, 0.0, 16384.0, 16384.0),
                );
            }
            RenderCommand::Rect {
                x,
                y,
                width,
                height,
                r,
                g,
                b,
                a,
            } => {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(*r, *g, *b, *a),
                    None,
                    &Rect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                    ),
                );
            }
            RenderCommand::RoundedRect {
                x,
                y,
                width,
                height,
                radius,
                r,
                g,
                b,
                a,
            } => {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(*r, *g, *b, *a),
                    None,
                    &RoundedRect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                        *radius as f64,
                    ),
                );
            }
            RenderCommand::Line {
                x1,
                y1,
                x2,
                y2,
                thickness,
                r,
                g,
                b,
                a,
            } => {
                scene.stroke(
                    &Stroke::new(*thickness as f64),
                    Affine::IDENTITY,
                    Color::from_rgba8(*r, *g, *b, *a),
                    None,
                    &Line::new((*x1 as f64, *y1 as f64), (*x2 as f64, *y2 as f64)),
                );
            }
            RenderCommand::Text {
                x,
                y,
                value,
                font_size,
                r,
                g,
                b,
                a,
            } => {
                let w = value.len() as f64 * (*font_size as f64 * 0.55);
                let h = *font_size as f64;
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(*r, *g, *b, *a),
                    None,
                    &Rect::new(*x as f64, *y as f64, *x as f64 + w, *y as f64 + h),
                );
            }
            RenderCommand::Image {
                x,
                y,
                width,
                height,
                ..
            } => {
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgba8(42, 42, 58, 255),
                    None,
                    &Rect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                    ),
                );
            }
            RenderCommand::Clip { .. } | RenderCommand::RestoreClip => {}
        }
    }

    // ── Chrome ────────────────────────────────────────────────────────────────

    fn build_chrome(
        scene: &mut Scene,
        state: &BrowserToolbarState,
        loading: BrowserLoadingState,
        width: f64,
    ) {
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            COL_TOOLBAR,
            None,
            &Rect::new(0.0, 0.0, width, TOOLBAR_H),
        );
        let accent = if loading == BrowserLoadingState::Loading {
            COL_ACCENT_LOAD
        } else {
            COL_ACCENT_IDLE
        };
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            accent,
            None,
            &Rect::new(0.0, TOOLBAR_H - ACCENT_H, width, TOOLBAR_H),
        );

        Self::draw_button(scene, BTN_BACK_X, state.can_go_back());
        Self::draw_arrow_left(scene, BTN_BACK_X, state.can_go_back());
        Self::draw_button(scene, BTN_FWD_X, state.can_go_forward());
        Self::draw_arrow_right(scene, BTN_FWD_X, state.can_go_forward());
        Self::draw_button(scene, BTN_RELOAD_X, true);
        Self::draw_reload_icon(scene, BTN_RELOAD_X, loading != BrowserLoadingState::Loading);

        let addr_color = if state.address_input_active() {
            COL_ADDR_ACTIVE
        } else {
            COL_ADDR
        };
        let addr_w = (width - ADDR_X - ADDR_MARGIN_RIGHT).max(0.0);
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            addr_color,
            None,
            &RoundedRect::new(
                ADDR_X,
                ADDR_Y,
                ADDR_X + addr_w,
                ADDR_Y + ADDR_H,
                ADDR_RADIUS,
            ),
        );
        let url = state.address_value();
        if !url.is_empty() {
            let text_w = (url.len() as f64 * 7.0).min(addr_w - 16.0).max(0.0);
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                COL_ADDR_TEXT,
                None,
                &Rect::new(
                    ADDR_X + 8.0,
                    ADDR_Y + 10.0,
                    ADDR_X + 8.0 + text_w,
                    ADDR_Y + 20.0,
                ),
            );
        }
    }

    fn draw_button(scene: &mut Scene, x: f64, enabled: bool) {
        let color = if enabled { COL_BTN_ON } else { COL_BTN_OFF };
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            color,
            None,
            &RoundedRect::new(x, BTN_Y, x + BTN_W, BTN_Y + BTN_H, BTN_RADIUS),
        );
    }

    fn draw_arrow_left(scene: &mut Scene, btn_x: f64, enabled: bool) {
        let color = if enabled { COL_ICON_ON } else { COL_ICON_OFF };
        let cx = btn_x + BTN_W / 2.0;
        let cy = BTN_Y + BTN_H / 2.0;
        let mut p = BezPath::new();
        p.move_to(Point::new(cx + 5.0, cy - 6.0));
        p.line_to(Point::new(cx - 4.0, cy));
        p.line_to(Point::new(cx + 5.0, cy + 6.0));
        scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &p);
    }

    fn draw_arrow_right(scene: &mut Scene, btn_x: f64, enabled: bool) {
        let color = if enabled { COL_ICON_ON } else { COL_ICON_OFF };
        let cx = btn_x + BTN_W / 2.0;
        let cy = BTN_Y + BTN_H / 2.0;
        let mut p = BezPath::new();
        p.move_to(Point::new(cx - 5.0, cy - 6.0));
        p.line_to(Point::new(cx + 4.0, cy));
        p.line_to(Point::new(cx - 5.0, cy + 6.0));
        scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &p);
    }

    fn draw_reload_icon(scene: &mut Scene, btn_x: f64, active: bool) {
        let color = if active { COL_ICON_ON } else { COL_ICON_OFF };
        let cx = btn_x + BTN_W / 2.0;
        let cy = BTN_Y + BTN_H / 2.0;
        if !active {
            scene.stroke(
                &Stroke::new(2.0),
                Affine::IDENTITY,
                color,
                None,
                &Line::new((cx - 5.0, cy - 5.0), (cx + 5.0, cy + 5.0)),
            );
            scene.stroke(
                &Stroke::new(2.0),
                Affine::IDENTITY,
                color,
                None,
                &Line::new((cx + 5.0, cy - 5.0), (cx - 5.0, cy + 5.0)),
            );
        } else {
            let mut p = BezPath::new();
            p.move_to(Point::new(cx + 6.0, cy - 2.0));
            p.line_to(Point::new(cx + 6.0, cy - 6.0));
            p.line_to(Point::new(cx, cy - 6.0));
            p.line_to(Point::new(cx - 5.0, cy - 3.0));
            p.line_to(Point::new(cx - 6.0, cy));
            p.line_to(Point::new(cx - 5.0, cy + 3.0));
            p.line_to(Point::new(cx, cy + 6.0));
            p.line_to(Point::new(cx + 5.0, cy + 3.0));
            scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &p);
            let mut a = BezPath::new();
            a.move_to(Point::new(cx + 3.0, cy - 8.0));
            a.line_to(Point::new(cx + 6.0, cy - 2.0));
            a.line_to(Point::new(cx + 9.0, cy - 6.0));
            scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &a);
        }
    }
}
