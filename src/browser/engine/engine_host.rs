use tokio::sync::mpsc;

use crate::browser::{
    engine::{BezotEngine, BrowserEngine, EngineKind},
    network::NetworkResponse,
};

pub(crate) struct EngineHost {
    bezot_engine: BezotEngine,
}

impl EngineHost {
    pub(crate) fn new(network_sender: mpsc::Sender<NetworkResponse>) -> Self {
        Self {
            bezot_engine: BezotEngine::new(network_sender),
        }
    }

    pub(crate) fn active_engine(&self) -> &dyn BrowserEngine {
        &self.bezot_engine
    }

    pub(crate) fn active_engine_mut(&mut self) -> &mut dyn BrowserEngine {
        &mut self.bezot_engine
    }

    pub(crate) fn active_kind(&self) -> EngineKind {
        EngineKind::Bezot
    }

    pub(crate) fn bezot_engine(&self) -> &BezotEngine {
        &self.bezot_engine
    }

    pub(crate) fn bezot_engine_mut(&mut self) -> &mut BezotEngine {
        &mut self.bezot_engine
    }
}
