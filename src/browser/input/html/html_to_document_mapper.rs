use crate::browser::{
    document::{DocumentModel, DocumentNode},
    input::html::HtmlDocument,
};

pub(crate) struct HtmlToDocumentMapper;

impl HtmlToDocumentMapper {
    pub(crate) fn map(html_document: &HtmlDocument) -> DocumentModel {
        let mut children = Vec::new();

        children.push(DocumentNode::heading(1, html_document.title()));

        for block in html_document.text_blocks() {
            children.push(DocumentNode::paragraph(block));
        }

        DocumentModel::new(html_document.title(), DocumentNode::block("body", children))
    }
}
