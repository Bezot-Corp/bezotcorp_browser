use servo::ScreenGeometry;
use webrender_api::units::{DeviceIntPoint, DeviceIntRect, DeviceIntSize};
use winit::dpi::PhysicalSize;

use crate::browser::servo_app::AppState;

impl AppState {
    pub(crate) fn resize_layout(&self, width: u32, height: u32) {
        self.layout.borrow_mut().resize(width, height);
    }

    pub(crate) fn content_size(&self) -> PhysicalSize<u32> {
        let layout = self.layout.borrow();

        PhysicalSize::new(layout.content_width(), layout.content_height())
    }

    pub(crate) fn screen_geometry(&self) -> ScreenGeometry {
        let layout = self.layout.borrow();

        let full_size = DeviceIntSize::new(layout.width() as i32, layout.height() as i32);
        let content_size = DeviceIntSize::new(
            layout.content_width() as i32,
            layout.content_height() as i32,
        );

        let content_min = DeviceIntPoint::new(layout.content_x() as i32, layout.content_y() as i32);
        let content_max = DeviceIntPoint::new(
            layout.content_x() as i32 + layout.content_width() as i32,
            layout.content_y() as i32 + layout.content_height() as i32,
        );

        ScreenGeometry {
            size: full_size,
            available_size: content_size,
            window_rect: DeviceIntRect::new(content_min, content_max),
        }
    }
}
