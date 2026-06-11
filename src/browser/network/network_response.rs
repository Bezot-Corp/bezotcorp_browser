#[derive(Debug)]
pub(crate) enum NetworkResponse {
    Html {
        request_id: u64,
        url: String,
        html: String,
    },
    Error {
        request_id: u64,
        url: String,
        reason: String,
    },
}

impl NetworkResponse {
    pub(crate) fn request_id(&self) -> u64 {
        match self {
            Self::Html { request_id, .. } => *request_id,
            Self::Error { request_id, .. } => *request_id,
        }
    }

    pub(crate) fn url(&self) -> &str {
        match self {
            Self::Html { url, .. } => url,
            Self::Error { url, .. } => url,
        }
    }
}
