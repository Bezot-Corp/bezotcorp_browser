use crate::browser::input::html::HtmlDocument;

pub(crate) struct HtmlParser;

impl HtmlParser {
    pub(crate) fn parse(source: &str) -> HtmlDocument {
        let title = Self::extract_title(source).unwrap_or_else(|| "HTML Document".to_string());
        let text_blocks = Self::extract_text_blocks(source);

        HtmlDocument::new(title, text_blocks)
    }

    fn extract_title(source: &str) -> Option<String> {
        let lower = source.to_lowercase();
        let start = lower.find("<title>")?;
        let end = lower.find("</title>")?;

        if end <= start {
            return None;
        }

        let raw = &source[start + "<title>".len()..end];
        let title = Self::normalize_text(raw);

        if title.is_empty() { None } else { Some(title) }
    }

    fn extract_text_blocks(source: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut current = String::new();
        let mut inside_tag = false;

        for character in source.chars() {
            match character {
                '<' => {
                    inside_tag = true;
                    Self::flush_text(&mut current, &mut blocks);
                }
                '>' => {
                    inside_tag = false;
                }
                _ if !inside_tag => {
                    current.push(character);
                }
                _ => {}
            }
        }

        Self::flush_text(&mut current, &mut blocks);

        blocks
    }

    fn flush_text(current: &mut String, blocks: &mut Vec<String>) {
        let text = Self::normalize_text(current);

        if !text.is_empty() {
            blocks.push(text);
        }

        current.clear();
    }

    fn normalize_text(value: &str) -> String {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}
