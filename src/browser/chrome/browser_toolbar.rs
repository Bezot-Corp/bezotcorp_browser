use crate::browser::chrome::ToolbarAction;
use crate::browser::state::BrowserToolbarState;

#[derive(Debug, Clone)]
pub(crate) struct BrowserToolbar {
    state: BrowserToolbarState,
}

impl BrowserToolbar {
    pub(crate) fn new(state: BrowserToolbarState) -> Self {
        Self { state }
    }

    pub(crate) fn state(&self) -> &BrowserToolbarState {
        &self.state
    }

    pub(crate) fn can_execute(&self, action: ToolbarAction) -> bool {
        match action {
            ToolbarAction::Back => self.state.can_go_back(),
            ToolbarAction::Forward => self.state.can_go_forward(),
            ToolbarAction::Reload => true,
            ToolbarAction::FocusAddressBar => true,
        }
    }

    pub(crate) fn address_value(&self) -> &str {
        self.state.address_value()
    }

    pub(crate) fn address_input_active(&self) -> bool {
        self.state.address_input_active()
    }
}
