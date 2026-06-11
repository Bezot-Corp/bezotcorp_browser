use std::cell::RefCell;
use std::rc::Rc;

use servo::{Servo, WebView, WindowRenderingContext};
use winit::window::Window;

use crate::browser::navigation::{AddressInputState, NavigationState};
use crate::browser::state::{BrowserLoadingState, BrowserState};

pub(crate) struct AppState {
    pub(crate) window: Window,
    pub(crate) servo: Servo,
    pub(crate) rendering_context: Rc<WindowRenderingContext>,
    pub(crate) webviews: RefCell<Vec<WebView>>,
    pub(crate) navigation: RefCell<NavigationState>,
    pub(crate) address_input: RefCell<AddressInputState>,
    pub(crate) browser_state: RefCell<BrowserState>,
}

impl AppState {
    pub(crate) fn new(
        window: Window,
        servo: Servo,
        rendering_context: Rc<WindowRenderingContext>,
        initial_url: impl Into<String>,
    ) -> Self {
        let initial_url = initial_url.into();

        Self {
            window,
            servo,
            rendering_context,
            webviews: RefCell::new(Vec::new()),
            navigation: RefCell::new(NavigationState::new(initial_url.clone())),
            address_input: RefCell::new(AddressInputState::default()),
            browser_state: RefCell::new(BrowserState::new(initial_url)),
        }
    }

    pub(crate) fn current_webview(&self) -> Option<WebView> {
        self.webviews.borrow().last().cloned()
    }
}

impl servo::WebViewDelegate for AppState {
    fn notify_new_frame_ready(&self, _: WebView) {
        self.browser_state
            .borrow_mut()
            .set_loading_state(BrowserLoadingState::Idle);

        self.update_window_chrome();
        self.window.request_redraw();
    }
}
