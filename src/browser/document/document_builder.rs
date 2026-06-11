use crate::browser::{
    document::{DocumentModel, DocumentNode, InternalPage},
    input::{HtmlParser, HtmlToDocumentMapper},
};

pub(crate) struct DocumentBuilder;

impl DocumentBuilder {
    pub(crate) fn build_internal(page: InternalPage) -> DocumentModel {
        match page {
            InternalPage::Home => DocumentModel::new(
                "BCB Home",
                DocumentNode::block(
                    "body",
                    vec![
                        DocumentNode::text("Bienvenue dans BezotCorp Browser."),
                        DocumentNode::text("Le rendu maison BCB est actif."),
                    ],
                ),
            ),
            InternalPage::About => DocumentModel::new(
                "À propos",
                DocumentNode::block(
                    "body",
                    vec![
                        DocumentNode::text("BCB utilise un shell Rust contrôlé."),
                        DocumentNode::text("Servo reste disponible comme fallback optionnel."),
                    ],
                ),
            ),
            InternalPage::Debug => {
                let html = r#"
        <html>
            <head>
                <title>BCB HTML Debug</title>
            </head>
            <body>
                <p>HTML minimal parsé par BCB.</p>
                <p>Le mapper convertit HTML vers DocumentModel.</p>
            </body>
        </html>
    "#;

                let html_document = HtmlParser::parse(html);
                HtmlToDocumentMapper::map(&html_document)
            }
            InternalPage::Unknown(url) => DocumentModel::new(
                "Page interne inconnue",
                DocumentNode::block(
                    "body",
                    vec![DocumentNode::text(format!("URL interne inconnue : {url}"))],
                ),
            ),
        }
    }
}
