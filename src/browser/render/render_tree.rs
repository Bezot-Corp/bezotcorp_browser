use crate::browser::render::RenderCommand;

#[derive(Debug, Clone)]
pub(crate) struct RenderTree {
    pub(crate) commands: Vec<RenderCommand>,
}

impl RenderTree {
    pub(crate) fn new(commands: Vec<RenderCommand>) -> Self {
        Self { commands }
    }
}
