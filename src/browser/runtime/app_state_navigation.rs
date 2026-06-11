use crate::browser::{
    navigation::NavigationCommand, runtime::AppState, state::BrowserLoadingState,
};

impl AppState {
    pub(crate) fn navigate_to(&self, url: impl Into<String>) {
        let url = self.navigation.borrow_mut().navigate_to(url).to_string();
        self.load_url(url);
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

    pub(crate) fn stop_loading(&self) {
        self.browser_state
            .borrow_mut()
            .set_loading_state(BrowserLoadingState::Idle);
        self.update_window_chrome();
    }

    pub(crate) fn go_home(&self) {
        self.navigate_to("bcb://home");
    }

    fn load_url(&self, url: String) {
        let navigation = self.navigation.borrow();
        let mut browser_state = self.browser_state.borrow_mut();
        browser_state.navigate(url);
        browser_state.set_can_go_back(navigation.can_go_back());
        browser_state.set_can_go_forward(navigation.can_go_forward());
        drop(browser_state);
        drop(navigation);
        self.update_window_chrome();
    }
}
