use crate::browser::navigation::NavigationCommand;
use crate::browser::servo_app::AppState;
use crate::browser::state::BrowserLoadingState;

impl AppState {
    pub(crate) fn navigate_to(&self, url: impl Into<String>) {
        let url = self.navigation.borrow_mut().navigate_to(url).to_string();
        self.load_tracked_url(url);
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

        self.load_tracked_url(url);
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

        self.load_tracked_url(url);
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

        self.load_tracked_url(url);
    }

    fn load_tracked_url(&self, url: String) {
        {
            let navigation = self.navigation.borrow();

            let mut browser_state = self.browser_state.borrow_mut();
            browser_state.set_current_url(url.clone());
            browser_state.set_loading_state(BrowserLoadingState::Loading);
            browser_state.set_can_go_back(navigation.can_go_back());
            browser_state.set_can_go_forward(navigation.can_go_forward());
        }

        self.update_window_chrome();
        self.load_url(url);
    }

    fn load_url(&self, url: impl AsRef<str>) {
        if let Ok(parsed_url) = url::Url::parse(url.as_ref())
            && let Some(webview) = self.current_webview()
        {
            webview.load(parsed_url);
        }
    }
}
