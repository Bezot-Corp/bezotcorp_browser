#[derive(Debug, Clone, Default)]
pub(crate) struct BrowserTitle {
    value: String,
}

impl BrowserTitle {
    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn set(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}
