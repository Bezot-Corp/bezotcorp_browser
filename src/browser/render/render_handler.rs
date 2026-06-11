use servo::RenderingContext;
use winit::dpi::PhysicalSize;

use crate::browser::engine::EngineKind;
use crate::browser::runtime::ServoBrowserApp;

impl ServoBrowserApp {
    pub(super) fn spin_servo(&self) {
        if let Self::Running { state, .. } = self {
            let active_kind = state.browser_state.borrow().engine_state().active_kind();

            if active_kind == EngineKind::Servo
                && let Some(servo) = state.servo.as_ref()
            {
                servo.spin_event_loop();
            }
        }
    }

    pub(super) fn handle_redraw(&self) {
        if let Self::Running { state, .. } = self {
            let active_kind = state.browser_state.borrow().engine_state().active_kind();

            match active_kind {
                EngineKind::Bezot => {
                    state.render_bezot();
                }
                EngineKind::Servo => {
                    if let Some(webview) = state.webviews.borrow().last() {
                        state.render_chrome();
                        webview.paint();

                        if let Some(rendering_context) = state.rendering_context.as_ref() {
                            rendering_context.present();
                        }
                    }
                }
            }
        }
    }

    pub(super) fn handle_resize(&self, new_size: PhysicalSize<u32>) {
        if let Self::Running { state, .. } = self {
            state.resize_layout(new_size.width, new_size.height);

            let active_kind = state.browser_state.borrow().engine_state().active_kind();

            if active_kind == EngineKind::Servo
                && let Some(webview) = state.webviews.borrow().last()
            {
                webview.resize(state.content_size());
            }

            state.window.request_redraw();
        }
    }
}
