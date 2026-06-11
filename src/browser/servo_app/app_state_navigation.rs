use crate::browser::navigation::NavigationCommand;
use crate::browser::servo_app::AppState;

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

    fn load_url(&self, url: impl AsRef<str>) {
        if let Ok(parsed_url) = url::Url::parse(url.as_ref())
            && let Some(webview) = self.current_webview()
        {
            webview.load(parsed_url);
        }
    }
}
