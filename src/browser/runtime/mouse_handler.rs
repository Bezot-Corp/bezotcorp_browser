use winit::dpi::PhysicalPosition;
use winit::event::{MouseButton, MouseScrollDelta};

use crate::browser::runtime::AppState;
use crate::browser::state::BrowserLoadingState;

const TOOLBAR_H: f32 = 72.0;
const BTN_Y_MIN: f32 = 14.0;
const BTN_Y_MAX: f32 = 46.0;
const BTN_BACK_X_MIN: f32 = 8.0;
const BTN_BACK_X_MAX: f32 = 40.0;
const BTN_FWD_X_MIN: f32 = 48.0;
const BTN_FWD_X_MAX: f32 = 80.0;
const BTN_RELOAD_X_MIN: f32 = 88.0;
const BTN_RELOAD_X_MAX: f32 = 120.0;
const ADDR_X_MIN: f32 = 136.0;
const ADDR_Y_MIN: f32 = 14.0;
const ADDR_Y_MAX: f32 = 46.0;

impl AppState {
    pub(crate) fn handle_mouse_click(&self, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }
        let (px, py) = self.cursor_position();

        if py < TOOLBAR_H {
            self.handle_toolbar_click(px, py);
        } else {
            self.handle_content_click(px, py);
        }
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

    fn handle_toolbar_click(&self, px: f32, py: f32) {
        if py < BTN_Y_MIN || py > BTN_Y_MAX {
            return;
        }
        if px >= BTN_BACK_X_MIN && px <= BTN_BACK_X_MAX {
            self.go_back();
        } else if px >= BTN_FWD_X_MIN && px <= BTN_FWD_X_MAX {
            self.go_forward();
        } else if px >= BTN_RELOAD_X_MIN && px <= BTN_RELOAD_X_MAX {
            let loading = self.browser_state.borrow().loading_state();
            if loading == BrowserLoadingState::Loading {
                self.stop_loading();
            } else {
                self.reload();
            }
        } else if px >= ADDR_X_MIN && py >= ADDR_Y_MIN && py <= ADDR_Y_MAX {
            self.begin_address_input();
        }
        self.update_window_chrome();
        self.window.request_redraw();
    }

    fn handle_content_click(&self, _px: f32, _py: f32) {
        // TODO: hit test layout_tree pour les liens
    }
}
