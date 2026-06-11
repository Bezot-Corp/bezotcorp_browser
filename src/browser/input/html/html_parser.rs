use crate::browser::input::html::{HtmlDocument, HtmlElement};

const VOID_ELEMENTS: &[&str] = &[
    "area", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "wbr",
];
const SKIP_ELEMENTS: &[&str] = &["script", "style", "head", "noscript", "template"];
const BLOCK_ELEMENTS: &[&str] = &[
    "div",
    "section",
    "article",
    "main",
    "nav",
    "footer",
    "header",
    "ul",
    "ol",
    "li",
    "blockquote",
    "figure",
];

pub(crate) struct HtmlParser;

enum Token {
    Open(String, Vec<(String, String)>),
    Close(String),
    SelfClose(String, Vec<(String, String)>),
    Text(String),
}

enum Content {
    Text(String),
    Element(HtmlElement),
}

impl HtmlParser {
    pub(crate) fn parse(source: &str) -> HtmlDocument {
        let title = Self::extract_title(source).unwrap_or_else(|| "HTML Document".to_string());
        let body = Self::build_elements(&Self::tokenize(source));
        HtmlDocument::new(title, body)
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

    fn tokenize(source: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let bytes = source.as_bytes();
        let len = source.len();
        let mut i = 0;

        while i < len {
            if bytes[i] != b'<' {
                let start = i;
                while i < len && bytes[i] != b'<' {
                    i += 1;
                }
                let text = Self::normalize_text(&source[start..i]);
                if !text.is_empty() {
                    tokens.push(Token::Text(text));
                }
                continue;
            }
            i += 1;
            let start = i;
            let mut in_quote = false;
            let mut quote_char = b'"';
            while i < len {
                if in_quote {
                    if bytes[i] == quote_char {
                        in_quote = false;
                    }
                } else if bytes[i] == b'"' || bytes[i] == b'\'' {
                    in_quote = true;
                    quote_char = bytes[i];
                } else if bytes[i] == b'>' {
                    break;
                }
                i += 1;
            }
            if let Some(token) = Self::parse_tag(&source[start..i]) {
                tokens.push(token);
            }
            if i < len {
                i += 1;
            }
        }
        tokens
    }

    fn parse_tag(content: &str) -> Option<Token> {
        let content = content.trim();
        if content.starts_with('!') || content.starts_with("--") {
            return None;
        }

        if let Some(name_part) = content.strip_prefix('/') {
            let name = name_part.trim().split_whitespace().next()?.to_lowercase();
            return Some(Token::Close(name));
        }

        let self_closing = content.ends_with('/');
        let content = if self_closing {
            content[..content.len() - 1].trim()
        } else {
            content
        };
        let (name, rest) = content
            .split_once(|c: char| c.is_whitespace())
            .unwrap_or((content, ""));
        let name = name.to_lowercase();
        let attrs = Self::parse_attributes(rest);

        if self_closing || VOID_ELEMENTS.contains(&name.as_str()) {
            Some(Token::SelfClose(name, attrs))
        } else {
            Some(Token::Open(name, attrs))
        }
    }

    fn parse_attributes(src: &str) -> Vec<(String, String)> {
        let mut attrs = Vec::new();
        let mut chars = src.chars().peekable();
        loop {
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }
            let mut key = String::new();
            while let Some(&c) = chars.peek() {
                if c == '=' || c.is_whitespace() {
                    break;
                }
                key.push(chars.next().unwrap().to_ascii_lowercase());
            }
            if key.is_empty() {
                break;
            }
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }
            if chars.peek() != Some(&'=') {
                attrs.push((key, String::new()));
                continue;
            }
            chars.next();
            while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
                chars.next();
            }
            let mut value = String::new();
            match chars.peek().copied() {
                Some(q @ '"') | Some(q @ '\'') => {
                    chars.next();
                    for c in chars.by_ref() {
                        if c == q {
                            break;
                        }
                        value.push(c);
                    }
                }
                _ => {
                    while let Some(&c) = chars.peek() {
                        if c.is_whitespace() {
                            break;
                        }
                        value.push(chars.next().unwrap());
                    }
                }
            }
            attrs.push((key, value));
        }
        attrs
    }

    fn build_elements(tokens: &[Token]) -> Vec<HtmlElement> {
        let mut stack: Vec<(String, Vec<(String, String)>, Vec<Content>)> =
            vec![("root".into(), Vec::new(), Vec::new())];
        let mut skip_depth = 0usize;

        for token in tokens {
            match token {
                Token::Open(name, _) if SKIP_ELEMENTS.contains(&name.as_str()) => {
                    skip_depth += 1;
                }
                Token::Close(name) if skip_depth > 0 && SKIP_ELEMENTS.contains(&name.as_str()) => {
                    skip_depth = skip_depth.saturating_sub(1);
                }
                _ if skip_depth > 0 => {}
                Token::Open(name, attrs) => {
                    stack.push((name.clone(), attrs.clone(), Vec::new()));
                }
                Token::Close(name) => {
                    if let Some(pos) = stack.iter().rposition(|(t, _, _)| t == name) {
                        while stack.len() > pos + 1 {
                            let (tag, attrs, content) = stack.pop().unwrap();
                            if let Some(e) = Self::finalize(&tag, &attrs, content)
                                && let Some((_, _, p)) = stack.last_mut()
                            {
                                p.push(Content::Element(e));
                            }
                        }
                        let (tag, attrs, content) = stack.pop().unwrap();
                        if let Some(e) = Self::finalize(&tag, &attrs, content)
                            && let Some((_, _, p)) = stack.last_mut()
                        {
                            p.push(Content::Element(e));
                        }
                    }
                }
                Token::SelfClose(name, attrs) => {
                    if let Some(e) = Self::build_void(name, attrs)
                        && let Some((_, _, p)) = stack.last_mut()
                    {
                        p.push(Content::Element(e));
                    }
                }
                Token::Text(text) => {
                    if let Some((_, _, p)) = stack.last_mut() {
                        p.push(Content::Text(text.clone()));
                    }
                }
            }
        }

        let (_, _, root) = stack.remove(0);
        Self::collect_children(root)
    }

    fn finalize(
        tag: &str,
        attrs: &[(String, String)],
        content: Vec<Content>,
    ) -> Option<HtmlElement> {
        let has_elements = content.iter().any(|c| matches!(c, Content::Element(_)));
        let text: String = content
            .iter()
            .filter_map(|c| {
                if let Content::Text(t) = c {
                    Some(t.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        let text = text.trim().to_string();

        match tag {
            "h1" => Some(HtmlElement::Heading { level: 1, text }),
            "h2" => Some(HtmlElement::Heading { level: 2, text }),
            "h3" => Some(HtmlElement::Heading { level: 3, text }),
            "h4" => Some(HtmlElement::Heading { level: 4, text }),
            "h5" => Some(HtmlElement::Heading { level: 5, text }),
            "h6" => Some(HtmlElement::Heading { level: 6, text }),
            "p" | "li" => {
                if text.is_empty() {
                    None
                } else {
                    Some(HtmlElement::Paragraph(text))
                }
            }
            "a" => {
                let href = Self::get_attr(attrs, "href");
                if text.is_empty() {
                    None
                } else {
                    Some(HtmlElement::Link { href, text })
                }
            }
            t if BLOCK_ELEMENTS.contains(&t) || matches!(t, "body" | "html") => {
                if has_elements {
                    let children = Self::collect_children(content);
                    if children.is_empty() {
                        None
                    } else {
                        Some(HtmlElement::Block {
                            tag: tag.to_string(),
                            children,
                        })
                    }
                } else if !text.is_empty() {
                    Some(HtmlElement::Paragraph(text))
                } else {
                    None
                }
            }
            "root" | "head" | "title" => None,
            _ if has_elements => {
                let children = Self::collect_children(content);
                if children.is_empty() {
                    None
                } else {
                    Some(HtmlElement::Block {
                        tag: tag.to_string(),
                        children,
                    })
                }
            }
            _ => None,
        }
    }

    fn build_void(tag: &str, attrs: &[(String, String)]) -> Option<HtmlElement> {
        match tag {
            "hr" => Some(HtmlElement::HorizontalRule),
            "img" => Some(HtmlElement::Image {
                src: Self::get_attr(attrs, "src"),
                alt: Self::get_attr(attrs, "alt"),
            }),
            _ => None,
        }
    }

    fn collect_children(content: Vec<Content>) -> Vec<HtmlElement> {
        content
            .into_iter()
            .filter_map(|c| match c {
                Content::Element(e) => Some(e),
                Content::Text(t) => {
                    let t = t.trim().to_string();
                    if t.is_empty() {
                        None
                    } else {
                        Some(HtmlElement::Paragraph(t))
                    }
                }
            })
            .collect()
    }

    fn get_attr(attrs: &[(String, String)], key: &str) -> String {
        attrs
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .unwrap_or_default()
    }

    fn normalize_text(value: &str) -> String {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}
