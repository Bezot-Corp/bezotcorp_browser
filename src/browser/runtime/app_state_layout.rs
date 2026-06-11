use winit::dpi::PhysicalSize;

use crate::browser::runtime::AppState;

impl AppState {
    pub(crate) fn resize_layout(&self, width: u32, height: u32) {
        self.layout.borrow_mut().resize(width, height);
    }

    pub(crate) fn content_size(&self) -> PhysicalSize<u32> {
        let layout = self.layout.borrow();
        PhysicalSize::new(layout.content_width(), layout.content_height())
    }
}
