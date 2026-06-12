use vello::kurbo::{Line, Rect, RoundedRect};
use vello::peniko::Color;

use crate::browser::render::{RenderCommand, RenderTree};
use crate::browser::ui::component::{Component, RenderContext};

pub(crate) struct DocumentView<'a> {
    pub(crate) render_tree: &'a RenderTree,
}

impl<'a> Component for DocumentView<'a> {
    fn render(&self, cx: &mut RenderContext<'_>) {
        for command in &self.render_tree.commands {
            RenderCommandView { command }.render(cx);
        }
    }
}

struct RenderCommandView<'a> {
    command: &'a RenderCommand,
}

impl<'a> Component for RenderCommandView<'a> {
    fn render(&self, cx: &mut RenderContext<'_>) {
        match self.command {
            RenderCommand::Clear { r, g, b, a } => {
                cx.fill(
                    &Rect::new(0.0, 0.0, 16384.0, 16384.0),
                    Color::from_rgba8(*r, *g, *b, *a),
                );
            }
            RenderCommand::Rect {
                x,
                y,
                width,
                height,
                r,
                g,
                b,
                a,
            } => {
                cx.fill(
                    &Rect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                    ),
                    Color::from_rgba8(*r, *g, *b, *a),
                );
            }
            RenderCommand::RoundedRect {
                x,
                y,
                width,
                height,
                radius,
                r,
                g,
                b,
                a,
            } => {
                cx.fill(
                    &RoundedRect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                        *radius as f64,
                    ),
                    Color::from_rgba8(*r, *g, *b, *a),
                );
            }
            RenderCommand::Line {
                x1,
                y1,
                x2,
                y2,
                thickness,
                r,
                g,
                b,
                a,
            } => {
                cx.stroke_line(
                    *x1 as f64,
                    *y1 as f64,
                    *x2 as f64,
                    *y2 as f64,
                    *thickness as f64,
                    Color::from_rgba8(*r, *g, *b, *a),
                );
            }
            RenderCommand::Text {
                x,
                y,
                value,
                font_size,
                r,
                g,
                b,
                a,
            } => {
                cx.text(
                    value,
                    *x,
                    *y,
                    *font_size,
                    None,
                    Color::from_rgba8(*r, *g, *b, *a),
                );
            }
            RenderCommand::Image {
                x,
                y,
                width,
                height,
                ..
            } => {
                cx.fill(
                    &Rect::new(
                        *x as f64,
                        *y as f64,
                        (*x + *width) as f64,
                        (*y + *height) as f64,
                    ),
                    Color::from_rgba8(42, 42, 58, 255),
                );
            }
            RenderCommand::Clip { .. } | RenderCommand::RestoreClip => {}
        }
    }
}
