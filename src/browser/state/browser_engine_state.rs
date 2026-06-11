use tokio::sync::mpsc;

use crate::browser::{
    engine::{EngineHost, EngineKind},
    layout::Viewport,
    network::NetworkResponse,
    render::{RenderTree, Renderer},
};

pub(crate) struct BrowserEngineState {
    engine_host: EngineHost,
}

impl BrowserEngineState {
    pub(crate) fn new(network_sender: mpsc::Sender<NetworkResponse>) -> Self {
        Self {
            engine_host: EngineHost::new(network_sender),
        }
    }

    pub(crate) fn active_engine_name(&self) -> &'static str {
        self.engine_host.active_engine().name()
    }

    pub(crate) fn active_engine_url(&self) -> &str {
        self.engine_host.active_engine().current_url()
    }

    pub(crate) fn active_kind(&self) -> EngineKind {
        self.engine_host.active_kind()
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

    pub(crate) fn bezot_engine_mut(&mut self) -> &mut crate::browser::engine::BezotEngine {
        self.engine_host.bezot_engine_mut()
    }

    pub(crate) fn bezot_render_tree(&self, viewport: &Viewport) -> RenderTree {
        Renderer::build_tree(self.engine_host.bezot_engine().current_document(), viewport)
    }

    pub(crate) fn scroll_by(&mut self, dy: f32) {
        // TODO: propager le scroll au layout tree stocké
        let _ = dy;
    }
}
