use crate::browser::layout::{LayoutBox, Viewport};

#[derive(Debug, Clone)]
pub(crate) struct LayoutTree {
    boxes: Vec<LayoutBox>,
    content_height: f32,
}

impl LayoutTree {
    pub(crate) fn new(boxes: Vec<LayoutBox>, content_height: f32) -> Self {
        Self {
            boxes,
            content_height,
        }
    }

    pub(crate) fn boxes(&self) -> &[LayoutBox] {
        &self.boxes
    }

    pub(crate) fn boxes_mut(&mut self) -> &mut [LayoutBox] {
        &mut self.boxes
    }

    pub(crate) fn content_height(&self) -> f32 {
        self.content_height
    }

    pub(crate) fn visible_boxes(&self, viewport: &Viewport) -> Vec<&LayoutBox> {
        self.boxes
            .iter()
            .filter(|b| b.intersects_viewport(viewport))
            .collect()
    }

    pub(crate) fn hit_test(&self, px: f32, py: f32) -> Option<&LayoutBox> {
        self.boxes.iter().rev().find(|b| b.contains_point(px, py))
    }

    pub(crate) fn scroll_by(&mut self, dy: f32) {
        for b in &mut self.boxes {
            b.translate_y(dy);
        }
    }
}
