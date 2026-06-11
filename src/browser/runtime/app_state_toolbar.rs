use crate::browser::runtime::AppState;
use crate::browser::state::BrowserToolbarState;

impl AppState {
    pub(crate) fn toolbar_state(&self) -> BrowserToolbarState {
        let browser_state = self.browser_state.borrow();
        let address_input = self.address_input.borrow();

        BrowserToolbarState::new(
            browser_state.can_go_back(),
            browser_state.can_go_forward(),
            address_input.is_active(),
            address_input.value(),
        )
    }
}
