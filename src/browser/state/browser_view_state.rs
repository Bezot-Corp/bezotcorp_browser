use crate::browser::state::BrowserLoadingState;

#[derive(Debug, Clone)]
pub(crate) struct BrowserViewState {
    current_url: String,
    title: String,
    loading_state: BrowserLoadingState,
    can_go_back: bool,
    can_go_forward: bool,
    address_input_active: bool,
    address_input_value: String,
}

impl BrowserViewState {
    pub(crate) fn new(
        current_url: impl Into<String>,
        title: impl Into<String>,
        loading_state: BrowserLoadingState,
        can_go_back: bool,
        can_go_forward: bool,
        address_input_active: bool,
        address_input_value: impl Into<String>,
    ) -> Self {
        Self {
            current_url: current_url.into(),
            title: title.into(),
            loading_state,
            can_go_back,
            can_go_forward,
            address_input_active,
            address_input_value: address_input_value.into(),
        }
    }

    pub(crate) fn current_url(&self) -> &str {
        &self.current_url
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn loading_state(&self) -> BrowserLoadingState {
        self.loading_state
    }

    pub(crate) fn can_go_back(&self) -> bool {
        self.can_go_back
    }

    pub(crate) fn can_go_forward(&self) -> bool {
        self.can_go_forward
    }

    pub(crate) fn address_input_active(&self) -> bool {
        self.address_input_active
    }

    pub(crate) fn address_input_value(&self) -> &str {
        &self.address_input_value
    }
}
