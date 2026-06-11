use crate::browser::runtime::AppState;

impl AppState {
    pub(crate) fn begin_address_input(&self) {
        let current_url = self.navigation.borrow().current_url().to_string();
        self.address_input.borrow_mut().activate(&current_url);
        self.update_window_chrome();
    }

    pub(crate) fn is_address_input_active(&self) -> bool {
        self.address_input.borrow().is_active()
    }

    pub(crate) fn commit_address_input(&self) {
        let url = self.address_input.borrow().value().to_string();

        self.address_input.borrow_mut().deactivate();

        if !url.is_empty() {
            self.navigate_to(url);
        } else {
            self.update_window_chrome();
        }
    }

    pub(crate) fn cancel_address_input(&self) {
        self.address_input.borrow_mut().deactivate();
        self.update_window_chrome();
    }

    pub(crate) fn append_address_input(&self, character: char) {
        self.address_input.borrow_mut().append_char(character);
        self.update_window_chrome();
    }

    pub(crate) fn remove_last_address_input_character(&self) {
        self.address_input.borrow_mut().remove_last_char();
        self.update_window_chrome();
    }
}
