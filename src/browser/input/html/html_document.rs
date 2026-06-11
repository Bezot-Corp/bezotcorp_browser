use crate::browser::input::html::HtmlElement;

#[derive(Debug, Clone)]
pub(crate) struct HtmlDocument {
    title: String,
    body: Vec<HtmlElement>,
}

impl HtmlDocument {
    pub(crate) fn new(title: impl Into<String>, body: Vec<HtmlElement>) -> Self {
        Self {
            title: title.into(),
            body,
        }
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn body_elements(&self) -> &[HtmlElement] {
        &self.body
    }
}
