#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InternalPage {
    Home,
    About,
    Debug,
    Loading,
    Unknown(String),
}

impl InternalPage {
    pub(crate) fn from_url(url: &str) -> Self {
        match url.trim() {
            "bezot://home" | "bcb://home" => Self::Home,
            "bezot://about" | "bcb://about" => Self::About,
            "bezot://debug" | "bcb://debug" => Self::Debug,
            "bezot://loading" | "bcb://loading" => Self::Loading,
            other => Self::Unknown(other.to_string()),
        }
    }

    pub(crate) fn is_internal_url(url: &str) -> bool {
        let trimmed = url.trim();
        trimmed.starts_with("bcb://") || trimmed.starts_with("bezot://")
    }

    pub(crate) fn to_url(&self) -> &str {
        match self {
            Self::Home => "bcb://home",
            Self::About => "bcb://about",
            Self::Debug => "bcb://debug",
            Self::Loading => "bcb://loading",
            Self::Unknown(url) => url.as_str(),
        }
    }
}
