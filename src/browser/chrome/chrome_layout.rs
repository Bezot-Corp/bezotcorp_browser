use vello::kurbo::{Rect, RoundedRect};

pub(crate) const TOOLBAR_H: f64 = 72.0;
pub(crate) const ACCENT_H: f64 = 3.0;

pub(crate) const BTN_Y: f64 = 14.0;
pub(crate) const BTN_W: f64 = 32.0;
pub(crate) const BTN_H: f64 = 32.0;
pub(crate) const BTN_RADIUS: f64 = 6.0;
pub(crate) const BTN_BACK_X: f64 = 8.0;
pub(crate) const BTN_FWD_X: f64 = 48.0;
pub(crate) const BTN_RELOAD_X: f64 = 88.0;

pub(crate) const ADDR_X: f64 = 136.0;
pub(crate) const ADDR_Y: f64 = 14.0;
pub(crate) const ADDR_H: f64 = 32.0;
pub(crate) const ADDR_RADIUS: f64 = 8.0;
pub(crate) const ADDR_MARGIN_RIGHT: f64 = 16.0;
pub(crate) const ADDR_FONT_SIZE: f32 = 13.0;
pub(crate) const ADDR_PADDING_X: f64 = 10.0;
pub(crate) const ADDR_BASELINE_OFFSET: f64 = 22.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChromeZone {
    Back,
    Forward,
    Reload,
    Address,
    Content,
    Outside,
}

pub(crate) struct ChromeLayout;

impl ChromeLayout {
    pub(crate) fn hit_test(px: f32, py: f32, window_width: f32) -> ChromeZone {
        if py >= TOOLBAR_H as f32 {
            return ChromeZone::Content;
        }
        let btn_y_min = BTN_Y as f32;
        let btn_y_max = (BTN_Y + BTN_H) as f32;
        if py < btn_y_min || py > btn_y_max {
            return ChromeZone::Outside;
        }
        if Self::in_button(px, BTN_BACK_X) {
            return ChromeZone::Back;
        }
        if Self::in_button(px, BTN_FWD_X) {
            return ChromeZone::Forward;
        }
        if Self::in_button(px, BTN_RELOAD_X) {
            return ChromeZone::Reload;
        }
        let addr_max = window_width - ADDR_MARGIN_RIGHT as f32;
        if px >= ADDR_X as f32 && px <= addr_max {
            return ChromeZone::Address;
        }
        ChromeZone::Outside
    }

    pub(crate) fn toolbar_rect(width: f64) -> Rect {
        Rect::new(0.0, 0.0, width, TOOLBAR_H)
    }

    pub(crate) fn accent_rect(width: f64) -> Rect {
        Rect::new(0.0, TOOLBAR_H - ACCENT_H, width, TOOLBAR_H)
    }

    pub(crate) fn button_rect(x: f64) -> RoundedRect {
        RoundedRect::new(x, BTN_Y, x + BTN_W, BTN_Y + BTN_H, BTN_RADIUS)
    }

    pub(crate) fn button_center(x: f64) -> (f64, f64) {
        (x + BTN_W / 2.0, BTN_Y + BTN_H / 2.0)
    }

    pub(crate) fn address_rect(width: f64) -> RoundedRect {
        let w = (width - ADDR_X - ADDR_MARGIN_RIGHT).max(0.0);
        RoundedRect::new(ADDR_X, ADDR_Y, ADDR_X + w, ADDR_Y + ADDR_H, ADDR_RADIUS)
    }

    pub(crate) fn address_text_pos() -> (f32, f32) {
        (
            (ADDR_X + ADDR_PADDING_X) as f32,
            (ADDR_Y + ADDR_BASELINE_OFFSET) as f32,
        )
    }

    pub(crate) fn address_text_max_width(window_width: f64) -> f32 {
        ((window_width - ADDR_X - ADDR_MARGIN_RIGHT - ADDR_PADDING_X * 2.0).max(0.0)) as f32
    }

    fn in_button(px: f32, btn_x: f64) -> bool {
        px >= btn_x as f32 && px <= (btn_x + BTN_W) as f32
    }
}
