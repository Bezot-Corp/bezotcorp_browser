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
                        DocumentNode::heading(1, "BezotCorp Browser"),
                        DocumentNode::paragraph("Le rendu maison BCB est actif."),
                        DocumentNode::paragraph("Bienvenue dans votre navigateur Rust contrôlé."),
                    ],
                ),
            ),
            InternalPage::About => DocumentModel::new(
                "À propos — BCB",
                DocumentNode::block(
                    "body",
                    vec![
                        DocumentNode::heading(1, "À propos de BCB"),
                        DocumentNode::paragraph(
                            "BCB utilise un shell Rust contrôlé avec un pipeline document maison.",
                        ),
                        DocumentNode::heading(2, "Moteur"),
                        DocumentNode::paragraph(
                            "Servo reste disponible comme backend optionnel et fallback web.",
                        ),
                    ],
                ),
            ),
            InternalPage::Debug => {
                let html = r#"
                    <html>
                        <head><title>BCB HTML Debug</title></head>
                        <body>
                            <h1>Debug HTML</h1>
                            <p>HTML minimal parsé par BCB.</p>
                            <p>Le mapper convertit HTML vers DocumentModel.</p>
                        </body>
                    </html>
                "#;
                Self::build_from_html(html)
            }
            InternalPage::Loading => DocumentModel::new(
                "Chargement…",
                DocumentNode::block(
                    "body",
                    vec![
                        DocumentNode::heading(1, "Chargement en cours…"),
                        DocumentNode::paragraph("La page est en cours de récupération."),
                    ],
                ),
            ),
            InternalPage::Unknown(url) => Self::build_error(&url, "URL interne non reconnue"),
        }
    }

    pub(crate) fn build_from_html(html: &str) -> DocumentModel {
        let html_document = HtmlParser::parse(html);
        HtmlToDocumentMapper::map(&html_document)
    }

    pub(crate) fn build_error(url: &str, reason: &str) -> DocumentModel {
        DocumentModel::new(
            "Erreur",
            DocumentNode::block(
                "body",
                vec![
                    DocumentNode::heading(1, "Page introuvable"),
                    DocumentNode::paragraph(reason),
                    DocumentNode::paragraph(format!("URL : {url}")),
                ],
            ),
        )
    }
}
