use crate::browser::chrome::{BTN_BACK_X, BTN_FWD_X, BTN_RELOAD_X, ChromeLayout};
use crate::browser::state::{BrowserLoadingState, BrowserToolbarState};
use crate::browser::ui::chrome::{
    accent_bar::AccentBar,
    address_bar::AddressBar,
    nav_button::{NavButton, NavButtonKind},
};
use crate::browser::ui::component::{Component, RenderContext};

pub(crate) struct Toolbar<'a> {
    pub(crate) state: &'a BrowserToolbarState,
    pub(crate) loading: BrowserLoadingState,
    pub(crate) window_width: f64,
}

impl<'a> Component for Toolbar<'a> {
    fn render(&self, cx: &mut RenderContext<'_>) {
        cx.fill(
            &ChromeLayout::toolbar_rect(self.window_width),
            cx.theme.toolbar(),
        );

        AccentBar {
            loading: self.loading,
            width: self.window_width,
        }
        .render(cx);

        NavButton {
            kind: NavButtonKind::Back,
            enabled: self.state.can_go_back(),
            x: BTN_BACK_X,
        }
        .render(cx);
        NavButton {
            kind: NavButtonKind::Forward,
            enabled: self.state.can_go_forward(),
            x: BTN_FWD_X,
        }
        .render(cx);

        let reload_kind = if self.loading == BrowserLoadingState::Loading {
            NavButtonKind::Stop
        } else {
            NavButtonKind::Reload
        };
        NavButton {
            kind: reload_kind,
            enabled: true,
            x: BTN_RELOAD_X,
        }
        .render(cx);

        AddressBar {
            value: self.state.address_value(),
            active: self.state.address_input_active(),
            window_width: self.window_width,
        }
        .render(cx);
    }
}
