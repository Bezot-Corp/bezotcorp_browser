use crate::browser::servo_app::{Waker, WakerEvent};
use crate::browser::shortcuts::ShortcutManager;
use servo::RenderingContext;
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::ModifiersState;
use winit::window::WindowId;

pub(crate) enum ServoBrowserApp {
    Initial {
        waker: Waker,
        initial_url: String,
        modifiers: ModifiersState,
    },
    Running {
        state: std::rc::Rc<crate::browser::servo_app::AppState>,
        modifiers: ModifiersState,
        shortcut_manager: ShortcutManager,
    },
}

impl ServoBrowserApp {
    pub(crate) fn new(event_loop: &EventLoop<WakerEvent>, initial_url: impl Into<String>) -> Self {
        Self::Initial {
            waker: Waker::new(event_loop),
            initial_url: initial_url.into(),
            modifiers: ModifiersState::empty(),
        }
    }
}

impl ApplicationHandler<WakerEvent> for ServoBrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_browser_window(event_loop);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: WakerEvent) {
        self.spin_servo();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.spin_servo();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let Self::Running { state, .. } = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    webview.paint();
                    state.rendering_context.present();
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                if let Self::Running { state, .. } = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    let (delta_x, delta_y, mode) = match delta {
                        MouseScrollDelta::LineDelta(dx, dy) => (
                            (dx * 76.0) as f64,
                            (dy * 76.0) as f64,
                            servo::WheelMode::DeltaLine,
                        ),
                        MouseScrollDelta::PixelDelta(delta) => {
                            (delta.x, delta.y, servo::WheelMode::DeltaPixel)
                        }
                    };

                    webview.notify_input_event(servo::InputEvent::Wheel(servo::WheelEvent::new(
                        servo::WheelDelta {
                            x: delta_x,
                            y: delta_y,
                            z: 0.0,
                            mode,
                        },
                        webrender_api::units::DevicePoint::default().into(),
                    )));
                }
            }

            WindowEvent::Resized(new_size) => {
                if let Self::Running { state, .. } = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    webview.resize(new_size);
                }
            }

            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.update_modifiers(new_modifiers.state());
            }

            WindowEvent::KeyboardInput { event, .. } => {
                if let Self::Running { state, .. } = self
                    && state.is_address_input_active()
                {
                    self.handle_address_input(&event);
                } else {
                    self.handle_shortcut(&event);
                }
            }

            _ => {}
        }
    }
}
