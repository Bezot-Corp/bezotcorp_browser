#[derive(Debug, Clone)]
pub(crate) struct BrowserToolbarState {
    can_go_back: bool,
    can_go_forward: bool,
    is_loading: bool,
    address_input_active: bool,
    address_value: String,
}

impl BrowserToolbarState {
    pub(crate) fn new(
        can_go_back: bool,
        can_go_forward: bool,
        is_loading: bool,
        address_input_active: bool,
        address_value: impl Into<String>,
    ) -> Self {
        Self {
            can_go_back,
            can_go_forward,
            is_loading,
            address_input_active,
            address_value: address_value.into(),
        }
    }

    pub(crate) fn can_go_back(&self) -> bool {
        self.can_go_back
    }

    pub(crate) fn can_go_forward(&self) -> bool {
        self.can_go_forward
    }

    pub(crate) fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub(crate) fn address_input_active(&self) -> bool {
        self.address_input_active
    }

    pub(crate) fn address_value(&self) -> &str {
        &self.address_value
    }
}
