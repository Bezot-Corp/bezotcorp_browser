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
}
