#[derive(Debug, Clone)]
pub(crate) struct HtmlDocument {
    title: String,
    text_blocks: Vec<String>,
}

impl HtmlDocument {
    pub(crate) fn new(title: impl Into<String>, text_blocks: Vec<String>) -> Self {
        Self {
            title: title.into(),
            text_blocks,
        }
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn text_blocks(&self) -> &[String] {
        &self.text_blocks
    }
}
