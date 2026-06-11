use crate::browser::{
    document::DocumentModel,
    layout::{LayoutBox, LayoutBoxKind, LayoutBuilder, LayoutTree, Viewport},
    render::{RenderCommand, RenderTree},
};

const COLOR_BG: (u8, u8, u8, u8) = (18, 18, 22, 255);
const COLOR_TEXT: (u8, u8, u8, u8) = (220, 220, 225, 255);
const COLOR_HEADING: (u8, u8, u8, u8) = (255, 255, 255, 255);
const COLOR_LINK: (u8, u8, u8, u8) = (90, 150, 255, 255);
const COLOR_ALT: (u8, u8, u8, u8) = (150, 150, 160, 255);
const COLOR_RULE: (u8, u8, u8, u8) = (60, 60, 70, 255);

const FONT_SIZE_H1: f32 = 34.0;
const FONT_SIZE_H2: f32 = 30.0;
const FONT_SIZE_H3: f32 = 26.0;
const FONT_SIZE_BODY: f32 = 16.0;

pub(crate) struct Renderer;

impl Renderer {
    pub(crate) fn build_tree(document: &DocumentModel, viewport: &Viewport) -> RenderTree {
        let layout_tree = LayoutBuilder::build(document);
        Self::build_tree_from_layout(&layout_tree, viewport)
    }

    pub(crate) fn build_tree_from_layout(
        layout_tree: &LayoutTree,
        viewport: &Viewport,
    ) -> RenderTree {
        let visible_boxes = layout_tree.visible_boxes(viewport);
        Self::build_tree_from_visible_boxes(&visible_boxes)
    }

    fn build_tree_from_visible_boxes(visible_boxes: &[&LayoutBox]) -> RenderTree {
        let (r, g, b, a) = COLOR_BG;
        let mut commands = vec![RenderCommand::Clear { r, g, b, a }];
        for layout_box in visible_boxes {
            Self::push_layout_box(layout_box, &mut commands);
        }
        RenderTree::new(commands)
    }

    fn push_layout_box(layout_box: &LayoutBox, commands: &mut Vec<RenderCommand>) {
        match &layout_box.kind {
            LayoutBoxKind::Text(value) => {
                let (r, g, b, a) = COLOR_TEXT;
                commands.push(RenderCommand::Text {
                    x: layout_box.x,
                    y: layout_box.y,
                    value: value.clone(),
                    font_size: FONT_SIZE_BODY,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::Paragraph(value) => {
                let (r, g, b, a) = COLOR_TEXT;
                commands.push(RenderCommand::Text {
                    x: layout_box.x,
                    y: layout_box.y,
                    value: value.clone(),
                    font_size: FONT_SIZE_BODY,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::Heading { level, text } => {
                let font_size = Self::heading_font_size(*level);
                let (r, g, b, a) = COLOR_HEADING;
                commands.push(RenderCommand::Text {
                    x: layout_box.x,
                    y: layout_box.y,
                    value: text.clone(),
                    font_size,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::Link { href: _, text } => {
                let (r, g, b, a) = COLOR_LINK;
                commands.push(RenderCommand::Text {
                    x: layout_box.x,
                    y: layout_box.y,
                    value: text.clone(),
                    font_size: FONT_SIZE_BODY,
                    r,
                    g,
                    b,
                    a,
                });
                commands.push(RenderCommand::Line {
                    x1: layout_box.x,
                    y1: layout_box.bottom(),
                    x2: layout_box.right(),
                    y2: layout_box.bottom(),
                    thickness: 1.0,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::Image { src: _, alt } => {
                let (r, g, b, a) = COLOR_ALT;
                commands.push(RenderCommand::Text {
                    x: layout_box.x,
                    y: layout_box.y,
                    value: format!("[{alt}]"),
                    font_size: FONT_SIZE_BODY,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::HorizontalRule => {
                let (r, g, b, a) = COLOR_RULE;
                commands.push(RenderCommand::Line {
                    x1: layout_box.x,
                    y1: layout_box.y,
                    x2: layout_box.right(),
                    y2: layout_box.y,
                    thickness: 1.0,
                    r,
                    g,
                    b,
                    a,
                });
            }
            LayoutBoxKind::Block => {}
        }
    }

    fn heading_font_size(level: u8) -> f32 {
        match level {
            1 => FONT_SIZE_H1,
            2 => FONT_SIZE_H2,
            _ => FONT_SIZE_H3,
        }
    }
}
