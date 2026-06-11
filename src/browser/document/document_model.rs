use super::DocumentNode;

#[derive(Debug, Clone)]
pub(crate) struct DocumentModel {
    pub(crate) title: String,
    pub(crate) root: DocumentNode,
}

impl DocumentModel {
    pub(crate) fn new(title: impl Into<String>, root: DocumentNode) -> Self {
        Self {
            title: title.into(),
            root,
        }
    }
}
