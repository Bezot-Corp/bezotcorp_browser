use crate::browser::runtime::AppState;
use crate::browser::state::BrowserViewState;

impl AppState {
    pub(crate) fn view_state(&self) -> BrowserViewState {
        let browser_state = self.browser_state.borrow();
        let address_input = self.address_input.borrow();

        BrowserViewState::new(
            browser_state.current_url(),
            browser_state.title().value(),
            browser_state.loading_state(),
            browser_state.can_go_back(),
            browser_state.can_go_forward(),
            address_input.is_active(),
            address_input.value(),
        )
    }
}
