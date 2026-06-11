use winit::dpi::PhysicalPosition;
use winit::event::{MouseButton, MouseScrollDelta};

use crate::browser::runtime::AppState;

impl AppState {
    pub(crate) fn handle_mouse_click(&self, button: MouseButton) {
        if button != MouseButton::Left {
            return;
        }
        let pos = self.address_input.borrow().cursor_position();
        let browser_state = self.browser_state.borrow();
        let viewport = self.content_viewport();
        let render_tree = browser_state.engine_state().bezot_render_tree(&viewport);
        drop(browser_state);

        // hit test → navigation sur les liens
        // TODO: construire layout_tree depuis render_tree pour hit_test
        let _ = (pos, render_tree);
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
    }

    pub(crate) fn handle_cursor_moved(&self, _position: PhysicalPosition<f64>) {
        // TODO: hover state, cursor change
    }
}
