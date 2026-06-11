use std::num::NonZeroUsize;
use std::sync::Arc;

use vello::kurbo::{Affine, BezPath, Line, Point, Rect, RoundedRect, Stroke};
use vello::peniko::{Color, Fill};
use vello::{AaConfig, RenderParams, RendererOptions, Scene};
use winit::window::Window;

use crate::browser::render::{RenderCommand, RenderTree};
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
const COL_TOOLBAR: Color = Color::rgba8(28, 28, 32, 255);
const COL_BTN_ON: Color = Color::rgba8(64, 64, 72, 255);
const COL_BTN_OFF: Color = Color::rgba8(38, 38, 44, 255);
const COL_ADDR: Color = Color::rgba8(44, 44, 52, 255);
const COL_ADDR_ACTIVE: Color = Color::rgba8(48, 56, 88, 255);
const COL_ICON_ON: Color = Color::rgba8(220, 220, 230, 255);
const COL_ICON_OFF: Color = Color::rgba8(90, 90, 100, 255);
const COL_ACCENT_LOAD: Color = Color::rgba8(0, 170, 210, 255);
const COL_ACCENT_IDLE: Color = Color::rgba8(60, 60, 70, 255);
const COL_ADDR_TEXT: Color = Color::rgba8(200, 200, 215, 255);
const COL_BG: Color = Color::rgba8(18, 18, 22, 255);

// ── Error ─────────────────────────────────────────────────────────────────────
#[derive(Debug, thiserror::Error)]
pub(crate) enum GpuRendererError {
    #[error("aucun adaptateur GPU disponible")]
    NoAdapter,
    #[error("erreur surface wgpu : {0}")]
    Surface(#[from] wgpu::CreateSurfaceError),
    #[error("erreur device wgpu : {0}")]
    Device(#[from] wgpu::RequestDeviceError),
    #[error("erreur vello : {0}")]
    Vello(#[from] vello::Error),
    #[error("erreur texture surface")]
    SurfaceTexture,
}

// ── Renderer ──────────────────────────────────────────────────────────────────
pub(crate) struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    renderer: vello::Renderer,
}

impl GpuRenderer {
    pub(crate) async fn new(window: Arc<Window>) -> Result<Self, GpuRendererError> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(GpuRendererError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("BCB GPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
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
                surface_format: Some(format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: NonZeroUsize::new(1),
            },
        )?;

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            renderer,
        })
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

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

        let surface_texture = self
            .surface
            .get_current_texture()
            .map_err(|_| GpuRendererError::SurfaceTexture)?;

        self.renderer.render_to_surface(
            &self.device,
            &self.queue,
            &scene,
            &surface_texture,
            &RenderParams {
                base_color: COL_BG,
                width: self.surface_config.width,
                height: self.surface_config.height,
                antialiasing_method: AaConfig::Msaa16,
            },
        )?;

        surface_texture.present();
        Ok(())
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
                    Color::rgba8(*r, *g, *b, *a),
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
                    Color::rgba8(*r, *g, *b, *a),
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
                let shape = RoundedRect::new(
                    *x as f64,
                    *y as f64,
                    (*x + *width) as f64,
                    (*y + *height) as f64,
                    *radius as f64,
                );
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::rgba8(*r, *g, *b, *a),
                    None,
                    &shape,
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
                    Color::rgba8(*r, *g, *b, *a),
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
                // TODO: cosmic-text shaping
                let w = value.len() as f64 * (*font_size as f64 * 0.55);
                let h = *font_size as f64;
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::rgba8(*r, *g, *b, *a),
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
                // TODO: image decoding + wgpu texture upload
                scene.fill(
                    Fill::NonZero,
                    Affine::IDENTITY,
                    Color::rgba8(42, 42, 58, 255),
                    None,
                    &Rect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                    ),
                );
            }
            RenderCommand::Clip { .. } | RenderCommand::RestoreClip => {
                // TODO: push_layer / pop_layer
            }
        }
    }

    // ── Chrome ────────────────────────────────────────────────────────────────

    fn build_chrome(
        scene: &mut Scene,
        state: &BrowserToolbarState,
        loading: BrowserLoadingState,
        width: f64,
    ) {
        // fond toolbar
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            COL_TOOLBAR,
            None,
            &Rect::new(0.0, 0.0, width, TOOLBAR_H),
        );

        // accent bas
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

        // bouton back
        Self::draw_button(scene, BTN_BACK_X, state.can_go_back());
        Self::draw_arrow_left(scene, BTN_BACK_X, state.can_go_back());

        // bouton forward
        Self::draw_button(scene, BTN_FWD_X, state.can_go_forward());
        Self::draw_arrow_right(scene, BTN_FWD_X, state.can_go_forward());

        // bouton reload/stop
        let reload_active = loading != BrowserLoadingState::Loading;
        Self::draw_button(scene, BTN_RELOAD_X, true);
        Self::draw_reload_icon(scene, BTN_RELOAD_X, reload_active);

        // barre d'adresse
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

        // texte URL (placeholder rect proportionnel)
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
        let mut path = BezPath::new();
        path.move_to(Point::new(cx + 5.0, cy - 6.0));
        path.line_to(Point::new(cx - 4.0, cy));
        path.line_to(Point::new(cx + 5.0, cy + 6.0));
        scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &path);
    }

    fn draw_arrow_right(scene: &mut Scene, btn_x: f64, enabled: bool) {
        let color = if enabled { COL_ICON_ON } else { COL_ICON_OFF };
        let cx = btn_x + BTN_W / 2.0;
        let cy = BTN_Y + BTN_H / 2.0;
        let mut path = BezPath::new();
        path.move_to(Point::new(cx - 5.0, cy - 6.0));
        path.line_to(Point::new(cx + 4.0, cy));
        path.line_to(Point::new(cx - 5.0, cy + 6.0));
        scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &path);
    }

    fn draw_reload_icon(scene: &mut Scene, btn_x: f64, active: bool) {
        let color = if active { COL_ICON_ON } else { COL_ICON_OFF };
        let cx = btn_x + BTN_W / 2.0;
        let cy = BTN_Y + BTN_H / 2.0;
        // simple croix pour stop, cercle partiel pour reload
        if !active {
            // stop — croix
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
            // reload — arc approximé avec deux lignes en V + flèche
            let mut path = BezPath::new();
            path.move_to(Point::new(cx, cy - 6.0));
            path.curve_to(
                Point::new(cx + 8.0, cy - 6.0),
                Point::new(cx + 8.0, cy + 6.0),
                Point::new(cx, cy + 6.0),
            );
            scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &path);
            // petite flèche en bas
            let mut arrow = BezPath::new();
            arrow.move_to(Point::new(cx - 4.0, cy + 4.0));
            arrow.line_to(Point::new(cx, cy + 8.0));
            arrow.line_to(Point::new(cx + 4.0, cy + 4.0));
            scene.stroke(&Stroke::new(2.0), Affine::IDENTITY, color, None, &arrow);
        }
    }
}
