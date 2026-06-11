use crate::browser::state::{BrowserLoadingState, BrowserTitle};

#[derive(Debug, Clone)]
pub(crate) struct BrowserState {
    current_url: String,
    title: BrowserTitle,
    loading_state: BrowserLoadingState,
}

impl BrowserState {
    pub(crate) fn new(initial_url: impl Into<String>) -> Self {
        Self {
            current_url: initial_url.into(),
            title: BrowserTitle::default(),
            loading_state: BrowserLoadingState::Idle,
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
}
