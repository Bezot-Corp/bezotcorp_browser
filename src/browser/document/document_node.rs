#[derive(Debug, Clone)]
pub(crate) enum DocumentNode {
    Text(String),
    Block {
        tag: String,
        children: Vec<DocumentNode>,
    },
}
