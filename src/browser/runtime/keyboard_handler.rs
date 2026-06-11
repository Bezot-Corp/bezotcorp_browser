use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, ModifiersState, NamedKey};

use crate::browser::runtime::BrowserApp;
use crate::browser::shortcuts::{ShortcutAction, WinitShortcutMapper};

impl BrowserApp {
    pub(crate) fn handle_keyboard(&self, event: &KeyEvent) {
        self.handle_shortcut(event);
        self.handle_address_input(event);
    }

    pub(crate) fn update_modifiers(&mut self, new_modifiers: ModifiersState) {
        self.modifiers = new_modifiers;
    }

    fn handle_shortcut(&self, event: &KeyEvent) {
        let Some(state) = self.app_state.as_ref() else {
            return;
        };

        if event.state != ElementState::Pressed {
            return;
        }

        if state.is_address_input_active() {
            return;
        }

        let Some(shortcut) = WinitShortcutMapper::from_key_event(event, self.modifiers) else {
            return;
        };

        let Some(action) = self
            .shortcut_manager
            .as_ref()
            .and_then(|m| m.action_for(&shortcut))
        else {
            return;
        };

        match action {
            ShortcutAction::Reload => state.reload(),
            ShortcutAction::Back => state.go_back(),
            ShortcutAction::Forward => state.go_forward(),
            ShortcutAction::OpenAddressBar => state.begin_address_input(),
        }
    }

    fn handle_address_input(&self, event: &KeyEvent) {
        let Some(state) = self.app_state.as_ref() else {
            return;
        };

        if !state.is_address_input_active() || event.state != ElementState::Pressed {
            return;
        }

        match &event.logical_key {
            Key::Named(NamedKey::Enter) => {
                state.commit_address_input();
            }
            Key::Named(NamedKey::Escape) => {
                state.cancel_address_input();
            }
            Key::Named(NamedKey::Backspace) => {
                state.remove_last_address_input_character();
            }
            Key::Character(text) => {
                for character in text.chars() {
                    if !character.is_control() {
                        state.append_address_input(character);
                    }
                }
            }
            _ => {}
        }
    }
}
