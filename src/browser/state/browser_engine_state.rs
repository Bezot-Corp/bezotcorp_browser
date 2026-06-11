use crate::browser::engine::{EngineHost, EngineKind};
use crate::browser::render::{RenderTree, Renderer};

pub(crate) struct BrowserEngineState {
    engine_host: EngineHost,
}

impl BrowserEngineState {
    pub(crate) fn new() -> Self {
        Self {
            engine_host: EngineHost::new(),
        }
    }

    pub(crate) fn active_engine_name(&self) -> &'static str {
        self.engine_host.active_engine().name()
    }

    pub(crate) fn active_engine_url(&self) -> &str {
        self.engine_host.active_engine().current_url()
    }

    pub(crate) fn load_url(&mut self, url: &str) {
        self.engine_host.active_engine_mut().load_url(url);
    }

    pub(crate) fn reload(&mut self) {
        self.engine_host.active_engine_mut().reload();
    }

    pub(crate) fn go_back(&mut self) {
        self.engine_host.active_engine_mut().go_back();
    }

    pub(crate) fn go_forward(&mut self) {
        self.engine_host.active_engine_mut().go_forward();
    }

    pub(crate) fn switch_engine(&mut self, kind: EngineKind) {
        self.engine_host.switch_to(kind);
    }

    pub(crate) fn active_kind(&self) -> EngineKind {
        self.engine_host.active_kind()
    }

    pub(crate) fn bezot_render_tree(&self) -> RenderTree {
        Renderer::build_tree(self.engine_host.bezot_engine().current_document())
    }
}
