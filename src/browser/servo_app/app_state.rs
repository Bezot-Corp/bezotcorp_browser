use std::cell::RefCell;
use std::rc::Rc;

use servo::{Servo, WebView, WindowRenderingContext};
use winit::window::Window;

use crate::browser::navigation::{AddressInputState, NavigationCommand, NavigationState};

pub(crate) struct AppState {
    pub(crate) window: Window,
    pub(crate) servo: Servo,
    pub(crate) rendering_context: Rc<WindowRenderingContext>,
    pub(crate) webviews: RefCell<Vec<WebView>>,
    pub(crate) navigation: RefCell<NavigationState>,
    pub(crate) address_input: RefCell<AddressInputState>,
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
            address_input: RefCell::new(AddressInputState::default()),
        }
    }

    pub(crate) fn current_webview(&self) -> Option<WebView> {
        self.webviews.borrow().last().cloned()
    }

    pub(crate) fn navigate_to(&self, url: impl Into<String>) {
        let url = self.navigation.borrow_mut().navigate_to(url).to_string();
        self.load_url(url);
    }

    pub(crate) fn begin_address_input(&self) {
        let current_url = self.navigation.borrow().current_url().to_string();
        self.address_input.borrow_mut().activate(&current_url);
    }

    pub(crate) fn is_address_input_active(&self) -> bool {
        self.address_input.borrow().is_active()
    }

    pub(crate) fn commit_address_input(&self) {
        let url = self.address_input.borrow().value().to_string();

        if !url.is_empty() {
            self.navigate_to(url);
        }

        self.address_input.borrow_mut().deactivate();
    }

    pub(crate) fn cancel_address_input(&self) {
        self.address_input.borrow_mut().deactivate();
    }

    pub(crate) fn reload(&self) {
        let Some(url) = self
            .navigation
            .borrow_mut()
            .apply_command(NavigationCommand::Reload)
            .map(str::to_string)
        else {
            return;
        };

        self.load_url(url);
    }

    pub(crate) fn go_back(&self) {
        let Some(url) = self
            .navigation
            .borrow_mut()
            .apply_command(NavigationCommand::Back)
            .map(str::to_string)
        else {
            return;
        };

        self.load_url(url);
    }

    pub(crate) fn go_forward(&self) {
        let Some(url) = self
            .navigation
            .borrow_mut()
            .apply_command(NavigationCommand::Forward)
            .map(str::to_string)
        else {
            return;
        };

        self.load_url(url);
    }

    fn load_url(&self, url: impl AsRef<str>) {
        if let Ok(parsed_url) = url::Url::parse(url.as_ref())
            && let Some(webview) = self.current_webview()
        {
            webview.load(parsed_url);
        }
    }

    pub(crate) fn append_address_input(&self, character: char) {
        self.address_input.borrow_mut().append_char(character);
    }

    pub(crate) fn remove_last_address_input_character(&self) {
        self.address_input.borrow_mut().remove_last_char();
    }
}

impl servo::WebViewDelegate for AppState {
    fn notify_new_frame_ready(&self, _: WebView) {
        self.window.request_redraw();
    }
}
