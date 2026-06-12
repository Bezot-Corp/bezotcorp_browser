use crate::browser::chrome::{ADDR_FONT_SIZE, ChromeLayout};
use crate::browser::ui::component::{Component, RenderContext};

pub(crate) struct AddressBar<'a> {
    pub(crate) value: &'a str,
    pub(crate) active: bool,
    pub(crate) window_width: f64,
}

impl<'a> Component for AddressBar<'a> {
    fn render(&self, cx: &mut RenderContext<'_>) {
        let color = if self.active {
            cx.theme.address_active()
        } else {
            cx.theme.address()
        };
        cx.fill(&ChromeLayout::address_rect(self.window_width), color);

        if !self.value.is_empty() {
            let (tx, ty) = ChromeLayout::address_text_pos();
            let max_w = ChromeLayout::address_text_max_width(self.window_width);
            let text_color = cx.theme.address_text();
            cx.text(self.value, tx, ty, ADDR_FONT_SIZE, Some(max_w), text_color);
        }
    }
}
