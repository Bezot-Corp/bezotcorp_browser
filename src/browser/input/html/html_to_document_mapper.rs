use crate::browser::{
    document::{DocumentModel, DocumentNode},
    input::html::{HtmlDocument, HtmlElement},
};

pub(crate) struct HtmlToDocumentMapper;

impl HtmlToDocumentMapper {
    pub(crate) fn map(html_document: &HtmlDocument) -> DocumentModel {
        let children = html_document
            .body_elements()
            .iter()
            .filter_map(Self::map_element)
            .collect();

        DocumentModel::new(html_document.title(), DocumentNode::block("body", children))
    }

    fn map_element(element: &HtmlElement) -> Option<DocumentNode> {
        match element {
            HtmlElement::Heading { level, text } => {
                Some(DocumentNode::heading(*level, text.clone()))
            }
            HtmlElement::Paragraph(text) => Some(DocumentNode::paragraph(text.clone())),
            HtmlElement::Link { href, text } => {
                Some(DocumentNode::link(href.clone(), text.clone()))
            }
            HtmlElement::Image { src, alt } => Some(DocumentNode::image(src.clone(), alt.clone())),
            HtmlElement::HorizontalRule => Some(DocumentNode::horizontal_rule()),
            HtmlElement::Block { tag, children } => {
                let mapped: Vec<DocumentNode> =
                    children.iter().filter_map(Self::map_element).collect();
                if mapped.is_empty() {
                    None
                } else {
                    Some(DocumentNode::block(tag.clone(), mapped))
                }
            }
            HtmlElement::Unknown => None,
        }
    }
}
