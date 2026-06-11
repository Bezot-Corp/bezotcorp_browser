use crate::browser::engine::BrowserEngine;

const DEFAULT_URL: &str = "https://servo.org";

pub struct ServoEngine {
    current_url: String,
}

impl ServoEngine {
    pub fn new() -> Self {
        Self {
            current_url: DEFAULT_URL.to_string(),
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

    pub fn is_servo_available() -> bool {
        use servo::{Servo, WebView, WebViewBuilder};

        let _servo_type = std::any::type_name::<Servo>();
        let _webview_type = std::any::type_name::<WebView>();
        let _webview_builder_type = std::any::type_name::<WebViewBuilder>();

        true
    }
}

impl BrowserEngine for ServoEngine {
    fn name(&self) -> &'static str {
        "servo"
    }

    fn current_url(&self) -> &str {
        &self.current_url
    }

    fn load_url(&mut self, url: &str) {
        self.current_url = Self::normalize_url(url);
    }

    fn reload(&mut self) {
        let current = self.current_url.clone();
        self.load_url(&current);
    }

    fn go_back(&mut self) {}

    fn go_forward(&mut self) {}
}
