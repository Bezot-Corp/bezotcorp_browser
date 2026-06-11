use super::DocumentNode;

#[derive(Debug, Clone)]
pub(crate) struct DocumentModel {
    pub(crate) title: String,
    pub(crate) root: DocumentNode,
}

impl DocumentModel {
    pub(crate) fn internal_page(title: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            root: DocumentNode::Block {
                tag: "body".to_string(),
                children: vec![DocumentNode::Text(text.into())],
            },
        }
    }
}
