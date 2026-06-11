use crate::browser::engine::{BezotEngine, BrowserEngine, EngineKind, ServoEngine};

pub(crate) struct EngineHost {
    active_kind: EngineKind,
    bezot_engine: BezotEngine,
    servo_engine: ServoEngine,
}

impl EngineHost {
    pub(crate) fn new() -> Self {
        Self {
            active_kind: EngineKind::Bezot,
            bezot_engine: BezotEngine::new(),
            servo_engine: ServoEngine::new(),
        }
    }

    pub(crate) fn active_kind(&self) -> EngineKind {
        self.active_kind
    }

    pub(crate) fn switch_to(&mut self, kind: EngineKind) {
        self.active_kind = kind;
    }

    pub(crate) fn active_engine(&self) -> &dyn BrowserEngine {
        match self.active_kind {
            EngineKind::Bezot => &self.bezot_engine,
            EngineKind::Servo => &self.servo_engine,
        }
    }

    pub(crate) fn active_engine_mut(&mut self) -> &mut dyn BrowserEngine {
        match self.active_kind {
            EngineKind::Bezot => &mut self.bezot_engine,
            EngineKind::Servo => &mut self.servo_engine,
        }
    }

    pub(crate) fn bezot_engine(&self) -> &BezotEngine {
        &self.bezot_engine
    }
}
