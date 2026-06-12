use crate::browser::chrome::ChromeLayout;
use crate::browser::state::BrowserLoadingState;
use crate::browser::ui::component::{Component, RenderContext};

pub(crate) struct AccentBar {
    pub(crate) loading: BrowserLoadingState,
    pub(crate) width: f64,
}

impl Component for AccentBar {
    fn render(&self, cx: &mut RenderContext<'_>) {
        let color = if self.loading == BrowserLoadingState::Loading {
            cx.theme.accent_loading()
        } else {
            cx.theme.accent_idle()
        };
        cx.fill(&ChromeLayout::accent_rect(self.width), color);
    }
}
