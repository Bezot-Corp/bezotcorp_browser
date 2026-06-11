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

        Self::push_node(&document.root, &mut commands, 24.0, 72.0);

        RenderTree::new(commands)
    }

    fn push_node(node: &DocumentNode, commands: &mut Vec<RenderCommand>, x: f32, y: f32) {
        match node {
            DocumentNode::Text(value) => {
                commands.push(RenderCommand::Text {
                    x,
                    y,
                    value: value.clone(),
                });
            }
            DocumentNode::Block { children, .. } => {
                let mut next_y = y;

                for child in children {
                    Self::push_node(child, commands, x, next_y);
                    next_y += 28.0;
                }
            }
        }
    }
}
