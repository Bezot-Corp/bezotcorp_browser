use serde::{Deserialize, Serialize};
use vello::peniko::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BrowserTheme {
    background: [u8; 4],
    toolbar: [u8; 4],
    button_on: [u8; 4],
    button_off: [u8; 4],
    icon_on: [u8; 4],
    icon_off: [u8; 4],
    address: [u8; 4],
    address_active: [u8; 4],
    address_text: [u8; 4],
    accent_loading: [u8; 4],
    accent_idle: [u8; 4],
}

impl Default for BrowserTheme {
    fn default() -> Self {
        Self {
            background: [18, 18, 22, 255],
            toolbar: [28, 28, 32, 255],
            button_on: [64, 64, 72, 255],
            button_off: [38, 38, 44, 255],
            icon_on: [220, 220, 230, 255],
            icon_off: [90, 90, 100, 255],
            address: [44, 44, 52, 255],
            address_active: [48, 56, 88, 255],
            address_text: [200, 200, 215, 255],
            accent_loading: [0, 170, 210, 255],
            accent_idle: [60, 60, 70, 255],
        }
    }
}

impl BrowserTheme {
    pub(crate) fn from_ron(s: &str) -> Result<Self, ron::error::SpannedError> {
        ron::from_str(s)
    }

    fn c(rgba: [u8; 4]) -> Color {
        Color::from_rgba8(rgba[0], rgba[1], rgba[2], rgba[3])
    }

    pub(crate) fn background(&self) -> Color {
        Self::c(self.background)
    }
    pub(crate) fn toolbar(&self) -> Color {
        Self::c(self.toolbar)
    }
    pub(crate) fn button_on(&self) -> Color {
        Self::c(self.button_on)
    }
    pub(crate) fn button_off(&self) -> Color {
        Self::c(self.button_off)
    }
    pub(crate) fn icon_on(&self) -> Color {
        Self::c(self.icon_on)
    }
    pub(crate) fn icon_off(&self) -> Color {
        Self::c(self.icon_off)
    }
    pub(crate) fn address(&self) -> Color {
        Self::c(self.address)
    }
    pub(crate) fn address_active(&self) -> Color {
        Self::c(self.address_active)
    }
    pub(crate) fn address_text(&self) -> Color {
        Self::c(self.address_text)
    }
    pub(crate) fn accent_loading(&self) -> Color {
        Self::c(self.accent_loading)
    }
    pub(crate) fn accent_idle(&self) -> Color {
        Self::c(self.accent_idle)
    }
}
