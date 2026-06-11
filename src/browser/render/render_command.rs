#[derive(Debug, Clone)]
pub(crate) enum RenderCommand {
    Clear { r: u8, g: u8, b: u8, a: u8 },
    Text { x: f32, y: f32, value: String },
}
