use crate::browser::{
    document::{DocumentModel, DocumentNode},
    render::{RenderCommand, RenderTree},
};

pub(crate) struct Renderer;

impl Renderer {
    pub(crate) fn build_tree(document: &DocumentModel) -> RenderTree {
        let mut commands = vec![RenderCommand::Clear {
            r: 18,
            g: 18,
            b: 22,
            a: 255,
        }];

        let mut cursor_y = 72.0;

        commands.push(RenderCommand::Text {
            x: 24.0,
            y: cursor_y,
            value: document.title.clone(),
        });

        cursor_y += 42.0;

        Self::push_node(&document.root, &mut commands, 24.0, &mut cursor_y);

        RenderTree::new(commands)
    }

    fn push_node(node: &DocumentNode, commands: &mut Vec<RenderCommand>, x: f32, y: &mut f32) {
        match node {
            DocumentNode::Text(value) => {
                commands.push(RenderCommand::Text {
                    x,
                    y: *y,
                    value: value.clone(),
                });

                *y += 24.0;
            }
            DocumentNode::Heading { level, text } => {
                commands.push(RenderCommand::Text {
                    x,
                    y: *y,
                    value: format!("H{level} {text}"),
                });

                *y += match level {
                    1 => 42.0,
                    2 => 36.0,
                    _ => 30.0,
                };
            }
            DocumentNode::Paragraph(value) => {
                commands.push(RenderCommand::Text {
                    x,
                    y: *y,
                    value: value.clone(),
                });

                *y += 28.0;
            }
            DocumentNode::Block { children, .. } => {
                for child in children {
                    Self::push_node(child, commands, x, y);
                }

                *y += 12.0;
            }
        }
    }
}
