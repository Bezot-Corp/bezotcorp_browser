use winit::dpi::PhysicalPosition;
use winit::event::{MouseButton, MouseScrollDelta};

use crate::browser::chrome::{ChromeLayout, ChromeZone};
use crate::browser::runtime::AppState;
use crate::browser::state::BrowserLoadingState;

impl AppState {
    pub(crate) fn handle_mouse_click(&self, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }
        let (px, py) = self.cursor_position();
        let width = self.window.inner_size().width as f32;

        match ChromeLayout::hit_test(px, py, width) {
            ChromeZone::Back => self.go_back(),
            ChromeZone::Forward => self.go_forward(),
            ChromeZone::Reload => {
                let loading = self.browser_state.borrow().loading_state();
                if loading == BrowserLoadingState::Loading {
                    self.stop_loading();
                } else {
                    self.reload();
                }
            }
            ChromeZone::Address => self.begin_address_input(),
            ChromeZone::Content => self.handle_content_click(px, py),
            ChromeZone::Outside => {}
        }
        self.update_window_chrome();
        self.window.request_redraw();
    }

    pub(crate) fn handle_scroll(&self, delta: MouseScrollDelta) {
        let dy = match delta {
            MouseScrollDelta::LineDelta(_, y) => y * 20.0,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
        };
        self.browser_state
            .borrow_mut()
            .engine_state_mut()
            .scroll_by(dy);
        self.window.request_redraw();
    }

    pub(crate) fn handle_cursor_moved(&self, position: PhysicalPosition<f64>) {
        *self.cursor.borrow_mut() = (position.x as f32, position.y as f32);
    }

    pub(crate) fn cursor_position(&self) -> (f32, f32) {
        *self.cursor.borrow()
    }

    fn handle_content_click(&self, _px: f32, _py: f32) {
        // TODO: hit test layout_tree pour les liens
    }
}
