pub struct BrowserToolbar;

impl BrowserToolbar {
    pub fn new() -> Self {
        Self
    }

    pub fn navigation_prompt_script(current_url: &str) -> String {
        format!(
            r#"
            const url = prompt("Navigate to:", "{}");
            if (url) {{
              const normalized = /^https?:\/\//.test(url.trim()) ? url.trim() : `https://${{url.trim()}}`;
              location.href = normalized;
            }}
            "#,
            escape_js_string(current_url)
        )
    }
}

fn escape_js_string(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
