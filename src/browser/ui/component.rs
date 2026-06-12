use vello::Scene;
use vello::kurbo::{Affine, BezPath, Line, Rect, RoundedRect, Stroke};
use vello::peniko::{Color, Fill};

use crate::browser::chrome::BrowserTheme;
use crate::browser::render::TextRenderer;

pub(crate) trait Component {
    fn render(&self, cx: &mut RenderContext<'_>);
}

pub(crate) struct RenderContext<'a> {
    pub(crate) scene: &'a mut Scene,
    pub(crate) text: &'a mut TextRenderer,
    pub(crate) theme: &'a BrowserTheme,
}

impl<'a> RenderContext<'a> {
    pub(crate) fn new(
        scene: &'a mut Scene,
        text: &'a mut TextRenderer,
        theme: &'a BrowserTheme,
    ) -> Self {
        Self { scene, text, theme }
    }

    pub(crate) fn fill(&mut self, shape: &impl vello::kurbo::Shape, color: Color) {
        self.scene
            .fill(Fill::NonZero, Affine::IDENTITY, color, None, shape);
    }

    pub(crate) fn stroke(&mut self, path: &BezPath, thickness: f64, color: Color) {
        self.scene
            .stroke(&Stroke::new(thickness), Affine::IDENTITY, color, None, path);
    }

    pub(crate) fn stroke_line(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        thickness: f64,
        color: Color,
    ) {
        self.scene.stroke(
            &Stroke::new(thickness),
            Affine::IDENTITY,
            color,
            None,
            &Line::new((x1, y1), (x2, y2)),
        );
    }

    pub(crate) fn text(
        &mut self,
        value: &str,
        x: f32,
        y: f32,
        font_size: f32,
        max_width: Option<f32>,
        color: Color,
    ) {
        self.text
            .draw(self.scene, value, x, y, font_size, max_width, color);
    }
}
