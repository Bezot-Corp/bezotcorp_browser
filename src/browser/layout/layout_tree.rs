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

    pub(crate) fn content_height(&self) -> f32 {
        self.content_height
    }

    pub(crate) fn visible_boxes(&self, viewport: &Viewport) -> Vec<&LayoutBox> {
        // TODO: replace temporary Vec allocation with iterator/visitor API.
        self.boxes
            .iter()
            .filter(|layout_box| layout_box.intersects_viewport(viewport))
            .collect()
    }

    pub(crate) fn hit_test_document_point(&self, px: f32, py: f32) -> Option<&LayoutBox> {
        self.boxes
            .iter()
            .rev()
            .find(|layout_box| layout_box.contains_document_point(px, py))
    }

    pub(crate) fn hit_test_screen_point(
        &self,
        viewport: &Viewport,
        px: f32,
        py: f32,
    ) -> Option<&LayoutBox> {
        let (document_x, document_y) = viewport.document_point_from_screen(px, py);
        self.hit_test_document_point(document_x, document_y)
    }
}
