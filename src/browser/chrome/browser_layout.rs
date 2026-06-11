pub(crate) const TOOLBAR_HEIGHT: u32 = 48;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BrowserLayout {
    width: u32,
    height: u32,
}

impl BrowserLayout {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn toolbar_height(&self) -> u32 {
        TOOLBAR_HEIGHT.min(self.height)
    }

    pub(crate) fn content_x(&self) -> u32 {
        0
    }

    pub(crate) fn content_y(&self) -> u32 {
        self.toolbar_height()
    }

    pub(crate) fn content_width(&self) -> u32 {
        self.width
    }

    pub(crate) fn content_height(&self) -> u32 {
        self.height.saturating_sub(self.toolbar_height())
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
