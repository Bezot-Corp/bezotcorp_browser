use crate::browser::{
    document::{DocumentModel, DocumentNode},
    layout::{LayoutBox, LayoutBoxKind, LayoutTree},
};

const CONTENT_X: f32 = 24.0;
const CONTENT_Y: f32 = 72.0;
const CONTENT_WIDTH: f32 = 960.0;
const BLOCK_INDENT: f32 = 16.0;
const FONT_SIZE_BODY: f32 = 16.0;
const CHAR_WIDTH_RATIO: f32 = 0.55;
const LINE_HEIGHT_RATIO: f32 = 1.4;
const HORIZONTAL_RULE_HEIGHT: f32 = 1.0;
const HORIZONTAL_RULE_MARGIN: f32 = 12.0;

struct LayoutHints {
    font_size: f32,
    margin_bottom: f32,
}

impl LayoutHints {
    fn for_heading(level: u8) -> Self {
        match level {
            1 => Self {
                font_size: 34.0,
                margin_bottom: 8.0,
            },
            2 => Self {
                font_size: 30.0,
                margin_bottom: 8.0,
            },
            _ => Self {
                font_size: 26.0,
                margin_bottom: 8.0,
            },
        }
    }

    fn for_paragraph() -> Self {
        Self {
            font_size: FONT_SIZE_BODY,
            margin_bottom: 6.0,
        }
    }

    fn for_text() -> Self {
        Self {
            font_size: FONT_SIZE_BODY,
            margin_bottom: 2.0,
        }
    }

    fn for_link() -> Self {
        Self {
            font_size: FONT_SIZE_BODY,
            margin_bottom: 2.0,
        }
    }
}

pub(crate) struct LayoutBuilder;

impl LayoutBuilder {
    pub(crate) fn build(document: &DocumentModel) -> LayoutTree {
        let mut boxes = Vec::new();
        let mut cursor_y = CONTENT_Y;

        let hints = LayoutHints::for_heading(1);
        boxes.push(LayoutBox::new(
            CONTENT_X,
            cursor_y,
            CONTENT_WIDTH,
            hints.font_size,
            LayoutBoxKind::Heading {
                level: 1,
                text: document.title.clone(),
            },
        ));
        cursor_y += hints.font_size + hints.margin_bottom + 6.0;

        Self::push_node(
            &document.root,
            &mut boxes,
            CONTENT_X,
            CONTENT_WIDTH,
            &mut cursor_y,
        );

        LayoutTree::new(boxes, cursor_y)
    }

    fn push_node(node: &DocumentNode, boxes: &mut Vec<LayoutBox>, x: f32, width: f32, y: &mut f32) {
        match node {
            DocumentNode::Text(value) => {
                let hints = LayoutHints::for_text();
                let height = Self::estimate_text_height(value, width, hints.font_size);
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    height,
                    LayoutBoxKind::Text(value.clone()),
                ));
                *y += height + hints.margin_bottom;
            }
            DocumentNode::Heading { level, text } => {
                let hints = LayoutHints::for_heading(*level);
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    hints.font_size,
                    LayoutBoxKind::Heading {
                        level: *level,
                        text: text.clone(),
                    },
                ));
                *y += hints.font_size + hints.margin_bottom;
            }
            DocumentNode::Paragraph(value) => {
                let hints = LayoutHints::for_paragraph();
                let height = Self::estimate_text_height(value, width, hints.font_size);
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    height,
                    LayoutBoxKind::Paragraph(value.clone()),
                ));
                *y += height + hints.margin_bottom;
            }
            DocumentNode::Link { href, text } => {
                let hints = LayoutHints::for_link();
                let height = Self::estimate_text_height(text, width, hints.font_size);
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    height,
                    LayoutBoxKind::Link {
                        href: href.clone(),
                        text: text.clone(),
                    },
                ));
                *y += height + hints.margin_bottom;
            }
            DocumentNode::Image { src, alt } => {
                let height = Self::estimate_text_height(alt, width, FONT_SIZE_BODY);
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    height,
                    LayoutBoxKind::Image {
                        src: src.clone(),
                        alt: alt.clone(),
                    },
                ));
                *y += height + 6.0;
            }
            DocumentNode::HorizontalRule => {
                *y += HORIZONTAL_RULE_MARGIN;
                boxes.push(LayoutBox::new(
                    x,
                    *y,
                    width,
                    HORIZONTAL_RULE_HEIGHT,
                    LayoutBoxKind::HorizontalRule,
                ));
                *y += HORIZONTAL_RULE_HEIGHT + HORIZONTAL_RULE_MARGIN;
            }
            DocumentNode::Block { children, .. } => {
                let child_x = x + BLOCK_INDENT;
                let child_width = (width - BLOCK_INDENT).max(0.0);
                for child in children {
                    Self::push_node(child, boxes, child_x, child_width, y);
                }
                *y += 12.0;
            }
        }
    }

    fn estimate_text_height(text: &str, width: f32, font_size: f32) -> f32 {
        let chars_per_line = ((width / (font_size * CHAR_WIDTH_RATIO)) as usize).max(1);
        let line_count = (text.len() / chars_per_line) + 1;
        line_count as f32 * (font_size * LINE_HEIGHT_RATIO)
    }
}
