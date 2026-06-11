use std::cell::RefCell;
use std::rc::Rc;

use servo::{Servo, WebView, WindowRenderingContext};
use winit::window::Window;

pub(crate) struct AppState {
    pub(crate) window: Window,
    pub(crate) servo: Servo,
    pub(crate) rendering_context: Rc<WindowRenderingContext>,
    pub(crate) webviews: RefCell<Vec<WebView>>,
}

impl AppState {
    pub(crate) fn new(
        window: Window,
        servo: Servo,
        rendering_context: Rc<WindowRenderingContext>,
    ) -> Self {
        Self {
            window,
            servo,
            rendering_context,
            webviews: RefCell::new(Vec::new()),
        }
    }
}

impl servo::WebViewDelegate for AppState {
    fn notify_new_frame_ready(&self, _: WebView) {
        self.window.request_redraw();
    }
}
