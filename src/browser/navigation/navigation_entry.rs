#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NavigationEntry {
    url: String,
    title: Option<String>,
}

impl NavigationEntry {
    pub(crate) fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: None,
        }
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub(crate) fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}
