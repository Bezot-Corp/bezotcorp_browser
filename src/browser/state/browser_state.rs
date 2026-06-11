use crate::browser::state::{BrowserLoadingState, BrowserTitle};

#[derive(Debug, Clone)]
pub(crate) struct BrowserState {
    current_url: String,
    title: BrowserTitle,
    loading_state: BrowserLoadingState,
    can_go_back: bool,
    can_go_forward: bool,
}

impl BrowserState {
    pub(crate) fn new(initial_url: impl Into<String>) -> Self {
        Self {
            current_url: initial_url.into(),
            title: BrowserTitle::default(),
            loading_state: BrowserLoadingState::Idle,
            can_go_back: false,
            can_go_forward: false,
        }
    }

    pub(crate) fn current_url(&self) -> &str {
        &self.current_url
    }

    pub(crate) fn set_current_url(&mut self, url: impl Into<String>) {
        self.current_url = url.into();
    }

    pub(crate) fn title(&self) -> &BrowserTitle {
        &self.title
    }

    pub(crate) fn title_mut(&mut self) -> &mut BrowserTitle {
        &mut self.title
    }

    pub(crate) fn loading_state(&self) -> BrowserLoadingState {
        self.loading_state
    }

    pub(crate) fn set_loading_state(&mut self, loading_state: BrowserLoadingState) {
        self.loading_state = loading_state;
    }

    pub(crate) fn can_go_back(&self) -> bool {
        self.can_go_back
    }

    pub(crate) fn set_can_go_back(&mut self, value: bool) {
        self.can_go_back = value;
    }

    pub(crate) fn can_go_forward(&self) -> bool {
        self.can_go_forward
    }

    pub(crate) fn set_can_go_forward(&mut self, value: bool) {
        self.can_go_forward = value;
    }
}
