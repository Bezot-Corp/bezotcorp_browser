use std::cell::RefCell;
use std::rc::Rc;

use servo::{ScreenGeometry, Servo, WebView, WindowRenderingContext};
use winit::window::Window;

use crate::browser::chrome::{BrowserLayout, ChromeRenderer};
use crate::browser::navigation::{AddressInputState, NavigationState};
use crate::browser::state::{BrowserLoadingState, BrowserState};

pub(crate) struct AppState {
    pub(crate) window: Rc<Window>,
    pub(crate) servo: Servo,
    pub(crate) rendering_context: Rc<WindowRenderingContext>,
    pub(crate) webviews: RefCell<Vec<WebView>>,
    pub(crate) navigation: RefCell<NavigationState>,
    pub(crate) address_input: RefCell<AddressInputState>,
    pub(crate) browser_state: RefCell<BrowserState>,
    pub(crate) layout: RefCell<BrowserLayout>,
    pub(crate) chrome_renderer: RefCell<Option<ChromeRenderer>>,
}

impl AppState {
    pub(crate) fn new(
        window: Rc<Window>,
        servo: Servo,
        rendering_context: Rc<WindowRenderingContext>,
        initial_url: impl Into<String>,
    ) -> Self {
        let initial_url = initial_url.into();
        let initial_size = window.inner_size();
        let chrome_renderer = ChromeRenderer::new(window.clone()).ok();

        Self {
            window,
            servo,
            rendering_context,
            webviews: RefCell::new(Vec::new()),
            navigation: RefCell::new(NavigationState::new(initial_url.clone())),
            address_input: RefCell::new(AddressInputState::default()),
            browser_state: RefCell::new(BrowserState::new(initial_url)),
            layout: RefCell::new(BrowserLayout::new(initial_size.width, initial_size.height)),
            chrome_renderer: RefCell::new(chrome_renderer),
        }
    }

    pub(crate) fn current_webview(&self) -> Option<WebView> {
        self.webviews.borrow().last().cloned()
    }

    pub(crate) fn render_chrome(&self) {
        let size = self.window.inner_size();
        let toolbar_state = self.toolbar_state();
        let loading_state = self.browser_state.borrow().loading_state();

        if let Some(renderer) = self.chrome_renderer.borrow_mut().as_mut() {
            let _ = renderer.render(size.width, size.height, &toolbar_state, loading_state);
        }
    }

    pub(crate) fn render_bezot(&self) {
        let size = self.window.inner_size();
        let toolbar_state = self.toolbar_state();

        let browser_state = self.browser_state.borrow();
        let loading_state = browser_state.loading_state();
        let render_tree = browser_state.engine_state().bezot_render_tree();
        drop(browser_state);

        if let Some(renderer) = self.chrome_renderer.borrow_mut().as_mut() {
            let _ = renderer.render_bezot(
                size.width,
                size.height,
                &toolbar_state,
                loading_state,
                &render_tree,
            );
        }
    }
}

impl servo::WebViewDelegate for AppState {
    fn screen_geometry(&self, _webview: WebView) -> Option<ScreenGeometry> {
        Some(self.screen_geometry())
    }

    fn notify_new_frame_ready(&self, _: WebView) {
        self.browser_state
            .borrow_mut()
            .set_loading_state(BrowserLoadingState::Idle);

        self.update_window_chrome();
        self.window.request_redraw();
    }
}
