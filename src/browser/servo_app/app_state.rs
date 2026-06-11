use std::cell::RefCell;
use std::rc::Rc;

use servo::{Servo, WebView, WindowRenderingContext};
use winit::window::Window;

use crate::browser::navigation::NavigationState;

pub(crate) struct AppState {
    pub(crate) window: Window,
    pub(crate) servo: Servo,
    pub(crate) rendering_context: Rc<WindowRenderingContext>,
    pub(crate) webviews: RefCell<Vec<WebView>>,
    pub(crate) navigation: RefCell<NavigationState>,
}

impl AppState {
    pub(crate) fn new(
        window: Window,
        servo: Servo,
        rendering_context: Rc<WindowRenderingContext>,
        initial_url: impl Into<String>,
    ) -> Self {
        Self {
            window,
            servo,
            rendering_context,
            webviews: RefCell::new(Vec::new()),
            navigation: RefCell::new(NavigationState::new(initial_url)),
        }
    }

    pub(crate) fn current_webview(&self) -> Option<WebView> {
        self.webviews.borrow().last().cloned()
    }

    pub(crate) fn navigate_to(&self, url: impl Into<String>) {
        let url = self.navigation.borrow_mut().navigate_to(url).to_string();

        if let Ok(parsed_url) = url::Url::parse(&url)
            && let Some(webview) = self.current_webview()
        {
            webview.load(parsed_url);
        }
    }

    pub(crate) fn reload(&self) {
        let current_url = self.navigation.borrow().current_url().to_string();

        if let Ok(parsed_url) = url::Url::parse(&current_url)
            && let Some(webview) = self.current_webview()
        {
            webview.load(parsed_url);
        }
    }
}

impl servo::WebViewDelegate for AppState {
    fn notify_new_frame_ready(&self, _: WebView) {
        self.window.request_redraw();
    }
}
