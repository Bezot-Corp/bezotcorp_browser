#[derive(Debug, Clone, Copy)]
pub(crate) struct Viewport {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) scroll_x: f32,
    pub(crate) scroll_y: f32,
}

impl Viewport {
    pub(crate) fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }

    pub(crate) fn with_scroll(mut self, scroll_x: f32, scroll_y: f32) -> Self {
        self.scroll_x = scroll_x.max(0.0);
        self.scroll_y = scroll_y.max(0.0);
        self
    }

    pub(crate) fn with_inset(&self, top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            x: self.x + left,
            y: self.y + top,
            width: (self.width - left - right).max(0.0),
            height: (self.height - top - bottom).max(0.0),
            scroll_x: self.scroll_x,
            scroll_y: self.scroll_y,
        }
    }

    pub(crate) fn document_x(&self) -> f32 {
        self.x + self.scroll_x
    }

    pub(crate) fn document_y(&self) -> f32 {
        self.y + self.scroll_y
    }

    pub(crate) fn document_right(&self) -> f32 {
        self.document_x() + self.width
    }

    pub(crate) fn document_bottom(&self) -> f32 {
        self.document_y() + self.height
    }

    pub(crate) fn screen_x_for_document_x(&self, x: f32) -> f32 {
        x - self.scroll_x
    }

    pub(crate) fn screen_y_for_document_y(&self, y: f32) -> f32 {
        y - self.scroll_y
    }

    pub(crate) fn document_point_from_screen(&self, px: f32, py: f32) -> (f32, f32) {
        (px + self.scroll_x, py + self.scroll_y)
    }

    pub(crate) fn contains_screen_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}
