use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::ModifiersState;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::browser::runtime::AppState;
use crate::browser::shortcuts::{Platform, ShortcutConfig, ShortcutManager};

const INITIAL_URL: &str = "bcb://home";
const WINDOW_TITLE: &str = "BezotCorp Browser";

pub(crate) struct BrowserApp {
    pub(super) app_state: Option<AppState>,
    pub(super) modifiers: ModifiersState,
    pub(super) shortcut_manager: Option<ShortcutManager>,
    tokio_runtime: Arc<tokio::runtime::Runtime>,
}

impl BrowserApp {
    pub(crate) fn new(tokio_runtime: Arc<tokio::runtime::Runtime>) -> Self {
        Self {
            app_state: None,
            modifiers: ModifiersState::default(),
            shortcut_manager: None,
            tokio_runtime,
        }
    }
}

impl ApplicationHandler for BrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.app_state.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title(WINDOW_TITLE)
            .with_inner_size(winit::dpi::PhysicalSize::new(1280, 800));

        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                tracing::error!("window creation failed: {e}");
                event_loop.exit();
                return;
            }
        };

        let app_state = self
            .tokio_runtime
            .block_on(AppState::new(window, INITIAL_URL));

        let shortcuts = include_str!("../../../config/keyboard_shortcuts.ron");
        if let Ok(config) = ShortcutConfig::from_ron_str(shortcuts) {
            self.shortcut_manager =
                Some(ShortcutManager::from_config(&config, Platform::current()));
        }

        app_state.update_window_chrome();
        app_state.window.request_redraw();
        self.app_state = Some(app_state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::ModifiersChanged(m) => {
                self.update_modifiers(m.state());
            }
            WindowEvent::Resized(size) => {
                if let Some(state) = &self.app_state {
                    state.resize(size.width, size.height);
                    state.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(state) = &self.app_state {
                    state.render();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard(&event);
            }
            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => {
                if btn_state == ElementState::Pressed {
                    if let Some(state) = &self.app_state {
                        state.handle_mouse_click(button);
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(state) = &self.app_state {
                    state.handle_scroll(delta);
                    state.window.request_redraw();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(state) = &self.app_state {
                    state.handle_cursor_moved(position);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.app_state {
            state.poll_network();
        }
    }
}
