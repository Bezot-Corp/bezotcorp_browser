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

#[cfg(test)]
mod tests {
    use super::escape_js_string;

    #[test]
    fn escapes_quotes() {
        assert_eq!(
            escape_js_string(r#"https://example.com/"x""#),
            r#"https://example.com/\"x\""#
        );
    }

    #[test]
    fn escapes_backslashes() {
        assert_eq!(
            escape_js_string(r#"https://example.com/a\b"#),
            r#"https://example.com/a\\b"#
        );
    }
}
