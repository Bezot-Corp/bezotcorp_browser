use crate::browser::layout::{LayoutBoxKind, Viewport};

#[derive(Debug, Clone)]
pub(crate) struct LayoutBox {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) kind: LayoutBoxKind,
}

impl LayoutBox {
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32, kind: LayoutBoxKind) -> Self {
        Self {
            x,
            y,
            width,
            height,
            kind,
        }
    }

    pub(crate) fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub(crate) fn right(&self) -> f32 {
        self.x + self.width
    }

    pub(crate) fn intersects_viewport(&self, viewport: &Viewport) -> bool {
        self.x < viewport.right()
            && self.right() > viewport.x
            && self.y < viewport.bottom()
            && self.bottom() > viewport.y
    }

    pub(crate) fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.right() && py >= self.y && py <= self.bottom()
    }

    pub(crate) fn translate_y(&mut self, dy: f32) {
        self.y += dy;
    }
}
