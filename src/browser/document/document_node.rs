#[derive(Debug, Clone)]
pub(crate) enum DocumentNode {
    Text(String),
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(String),
    Block {
        tag: String,
        children: Vec<DocumentNode>,
    },
    Link {
        href: String,
        text: String,
    },
    Image {
        src: String,
        alt: String,
    },
    HorizontalRule,
}

impl DocumentNode {
    pub(crate) fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub(crate) fn heading(level: u8, text: impl Into<String>) -> Self {
        Self::Heading {
            level,
            text: text.into(),
        }
    }

    pub(crate) fn paragraph(value: impl Into<String>) -> Self {
        Self::Paragraph(value.into())
    }

    pub(crate) fn block(tag: impl Into<String>, children: Vec<DocumentNode>) -> Self {
        Self::Block {
            tag: tag.into(),
            children,
        }
    }

    pub(crate) fn link(href: impl Into<String>, text: impl Into<String>) -> Self {
        Self::Link {
            href: href.into(),
            text: text.into(),
        }
    }

    pub(crate) fn image(src: impl Into<String>, alt: impl Into<String>) -> Self {
        Self::Image {
            src: src.into(),
            alt: alt.into(),
        }
    }

    pub(crate) fn horizontal_rule() -> Self {
        Self::HorizontalRule
    }
}
