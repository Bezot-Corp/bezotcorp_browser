use winit::event::MouseScrollDelta;

use crate::browser::engine::EngineKind;
use crate::browser::servo_app::ServoBrowserApp;

impl ServoBrowserApp {
    pub(super) fn handle_mouse_wheel(&self, delta: MouseScrollDelta) {
        if let Self::Running { state, .. } = self {
            let active_kind = state.browser_state.borrow().engine_state().active_kind();

            if active_kind != EngineKind::Servo {
                return;
            }

            let webview = state.current_webview();

            let Some(webview) = webview else {
                return;
            };

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
}
