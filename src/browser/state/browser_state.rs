use crate::browser::state::{BrowserEngineState, BrowserLoadingState, BrowserTitle};

pub(crate) struct BrowserState {
    current_url: String,
    title: BrowserTitle,
    loading_state: BrowserLoadingState,
    can_go_back: bool,
    can_go_forward: bool,
    engine_state: BrowserEngineState,
}

impl BrowserState {
    pub(crate) fn new(initial_url: impl Into<String>) -> Self {
        let initial_url = initial_url.into();

        let mut engine_state = BrowserEngineState::new();
        engine_state.load_url(&initial_url);

        Self {
            current_url: initial_url,
            title: BrowserTitle::default(),
            loading_state: BrowserLoadingState::Idle,
            can_go_back: false,
            can_go_forward: false,
            engine_state,
        }
    }

    pub(crate) fn current_url(&self) -> &str {
        &self.current_url
    }

    pub(crate) fn set_current_url(&mut self, url: impl Into<String>) {
        let url = url.into();

        self.engine_state.load_url(&url);
        self.current_url = url;
    }

    pub(crate) fn engine_state(&self) -> &BrowserEngineState {
        &self.engine_state
    }

    pub(crate) fn engine_state_mut(&mut self) -> &mut BrowserEngineState {
        &mut self.engine_state
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
