use crate::browser::engine::BrowserEngine;

pub(crate) struct ServoEngine {
    current_url: String,
}

impl ServoEngine {
    pub(crate) fn new(initial_url: impl Into<String>) -> Self {
        Self {
            current_url: initial_url.into(),
        }
    }

    pub(crate) fn current_url(&self) -> &str {
        &self.current_url
    }
}

impl BrowserEngine for ServoEngine {
    fn name(&self) -> &'static str {
        "servo"
    }

    fn load_url(&mut self, url: &str) {
        self.current_url = normalize_url(url);
    }

    fn reload(&mut self) {}

    fn go_back(&mut self) {}

    fn go_forward(&mut self) {}
}

fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}
