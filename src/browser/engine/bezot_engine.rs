use crate::browser::{document::DocumentModel, engine::BrowserEngine};

const DEFAULT_URL: &str = "bezot://home";

pub(crate) struct BezotEngine {
    current_url: String,
    current_document: DocumentModel,
}

impl BezotEngine {
    pub(crate) fn new() -> Self {
        Self {
            current_url: DEFAULT_URL.to_string(),
            current_document: Self::build_internal_page(DEFAULT_URL),
        }
    }

    pub(crate) fn current_document(&self) -> &DocumentModel {
        &self.current_document
    }

    fn build_internal_page(url: &str) -> DocumentModel {
        match url {
            "bezot://home" => DocumentModel::internal_page(
                "BezotCorp Browser",
                "Bienvenue dans le rendu maison BezotEngine.",
            ),
            "bezot://about" => DocumentModel::internal_page(
                "À propos",
                "Shell Rust contrôlé, renderer maison, Servo en fallback optionnel.",
            ),
            _ => DocumentModel::internal_page(
                "Page interne",
                format!("URL chargée par BezotEngine : {url}"),
            ),
        }
    }
}

impl BrowserEngine for BezotEngine {
    fn name(&self) -> &'static str {
        "bezot"
    }

    fn current_url(&self) -> &str {
        &self.current_url
    }

    fn load_url(&mut self, url: &str) {
        self.current_url = url.trim().to_string();
        self.current_document = Self::build_internal_page(&self.current_url);
    }

    fn reload(&mut self) {
        let current = self.current_url.clone();
        self.load_url(&current);
    }

    fn go_back(&mut self) {}

    fn go_forward(&mut self) {}
}
