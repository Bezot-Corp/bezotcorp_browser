use crate::browser::{
    document::{DocumentBuilder, DocumentModel, InternalPage},
    engine::BrowserEngine,
};

const DEFAULT_URL: &str = "bcb://home";

pub(crate) struct BezotEngine {
    current_url: String,
    current_document: DocumentModel,
}

impl BezotEngine {
    pub(crate) fn new() -> Self {
        Self {
            current_url: DEFAULT_URL.to_string(),
            current_document: DocumentBuilder::build_internal(InternalPage::Home),
        }
    }

    pub(crate) fn current_document(&self) -> &DocumentModel {
        &self.current_document
    }

    fn build_document(url: &str) -> DocumentModel {
        DocumentBuilder::build_internal(InternalPage::from_url(url))
    }
}

impl BrowserEngine for BezotEngine {
    fn name(&self) -> &'static str {
        "bcb"
    }

    fn current_url(&self) -> &str {
        &self.current_url
    }

    fn load_url(&mut self, url: &str) {
        self.current_url = url.trim().to_string();
        self.current_document = Self::build_document(&self.current_url);
    }

    fn reload(&mut self) {
        let current = self.current_url.clone();
        self.load_url(&current);
    }

    fn go_back(&mut self) {}

    fn go_forward(&mut self) {}
}
