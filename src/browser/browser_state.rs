const DEFAULT_URL: &str = "https://www.google.com";

#[derive(Debug, Clone)]
pub struct BrowserState {
    current_url: String,
}

impl BrowserState {
    pub fn new() -> Self {
        Self {
            current_url: DEFAULT_URL.to_string(),
        }
    }

    pub fn current_url(&self) -> &str {
        &self.current_url
    }

    pub fn navigate_to(&mut self, url: String) {
        self.current_url = normalize_url(&url);
    }
}

fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}
