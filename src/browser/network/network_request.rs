#[derive(Debug, Clone)]
pub(crate) struct NetworkRequest {
    pub(crate) url: String,
    pub(crate) request_id: u64,
}

impl NetworkRequest {
    pub(crate) fn new(url: impl Into<String>, request_id: u64) -> Self {
        Self {
            url: url.into(),
            request_id,
        }
    }
}
