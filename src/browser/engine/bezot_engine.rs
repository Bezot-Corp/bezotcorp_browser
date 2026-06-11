use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::mpsc;

use crate::browser::{
    document::{DocumentBuilder, DocumentModel, InternalPage},
    engine::BrowserEngine,
    network::{NetworkClient, NetworkRequest, NetworkResponse},
};

const DEFAULT_URL: &str = "bcb://home";

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(crate) struct BezotEngine {
    current_url: String,
    current_document: DocumentModel,
    pending_request_id: Option<u64>,
    network: Arc<NetworkClient>,
}

impl BezotEngine {
    pub(crate) fn new(network_sender: mpsc::Sender<NetworkResponse>) -> Self {
        let network = Arc::new(NetworkClient::new(network_sender));
        Self {
            current_url: DEFAULT_URL.to_string(),
            current_document: DocumentBuilder::build_internal(InternalPage::Home),
            pending_request_id: None,
            network,
        }
    }

    pub(crate) fn current_document(&self) -> &DocumentModel {
        &self.current_document
    }

    pub(crate) fn is_loading(&self) -> bool {
        self.pending_request_id.is_some()
    }

    pub(crate) fn apply_response(&mut self, response: NetworkResponse) {
        if self.pending_request_id != Some(response.request_id()) {
            return;
        }
        self.pending_request_id = None;
        self.current_url = response.url().to_string();
        self.current_document = match response {
            NetworkResponse::Html { html, url, .. } => {
                let doc = DocumentBuilder::build_from_html(&html);
                if doc.title.is_empty() {
                    DocumentBuilder::build_from_html(&html)
                } else {
                    doc
                }
            }
            NetworkResponse::Error { url, reason, .. } => {
                DocumentBuilder::build_error(&url, &reason)
            }
        };
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
        let url = url.trim().to_string();
        self.current_url = url.clone();

        if InternalPage::is_internal_url(&url) {
            self.pending_request_id = None;
            self.current_document = DocumentBuilder::build_internal(InternalPage::from_url(&url));
            return;
        }

        let request_id = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
        self.pending_request_id = Some(request_id);
        self.current_document = DocumentBuilder::build_internal(InternalPage::Loading);
        self.network.fetch(NetworkRequest::new(url, request_id));
    }

    fn reload(&mut self) {
        let url = self.current_url.clone();
        self.load_url(&url);
    }

    fn go_back(&mut self) {}

    fn go_forward(&mut self) {}
}
