#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InternalPage {
    Home,
    About,
    Debug,
    Unknown(String),
}

impl InternalPage {
    pub(crate) fn from_url(url: &str) -> Self {
        match url.trim() {
            "bezot://home" | "bcb://home" => Self::Home,
            "bezot://about" | "bcb://about" => Self::About,
            "bezot://debug" | "bcb://debug" => Self::Debug,
            other => Self::Unknown(other.to_string()),
        }
    }
}
