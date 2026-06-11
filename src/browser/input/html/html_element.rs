#[derive(Debug, Clone)]
pub(crate) enum HtmlElement {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(String),
    Link {
        href: String,
        text: String,
    },
    Image {
        src: String,
        alt: String,
    },
    HorizontalRule,
    Block {
        tag: String,
        children: Vec<HtmlElement>,
    },
    Unknown,
}
