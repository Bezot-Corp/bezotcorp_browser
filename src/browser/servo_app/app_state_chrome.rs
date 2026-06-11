use crate::browser::servo_app::AppState;
use crate::browser::state::BrowserLoadingState;

impl AppState {
    pub(crate) fn update_window_chrome(&self) {
        let view_state = self.view_state();
        let toolbar_state = self.toolbar_state();

        let loading_marker = match view_state.loading_state() {
            BrowserLoadingState::Idle => "",
            BrowserLoadingState::Loading => " • Loading",
        };

        let address = if toolbar_state.address_input_active() {
            toolbar_state.address_value()
        } else {
            view_state.current_url()
        };

        let input_marker = if toolbar_state.address_input_active() {
            " • Address"
        } else {
            ""
        };

        self.window.set_title(&format!(
            "BezotCorp Browser{loading_marker}{input_marker} — {address}"
        ));
    }
}
