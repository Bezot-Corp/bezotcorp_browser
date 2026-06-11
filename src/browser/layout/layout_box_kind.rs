#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LayoutBoxKind {
    Text(String),
    Heading { level: u8, text: String },
    Paragraph(String),
    Block,
    Link { href: String, text: String },
    Image { src: String, alt: String },
    HorizontalRule,
}
