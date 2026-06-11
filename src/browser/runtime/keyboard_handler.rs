use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{Key, ModifiersState, NamedKey};

use crate::browser::runtime::ServoBrowserApp;
use crate::browser::shortcuts::{ShortcutAction, WinitShortcutMapper};

impl ServoBrowserApp {
    pub(crate) fn handle_shortcut(&self, event: &KeyEvent) {
        let Self::Running {
            state,
            modifiers,
            shortcut_manager,
        } = self
        else {
            return;
        };

        if event.state != ElementState::Pressed {
            return;
        }

        if state.is_address_input_active() {
            return;
        }

        let Some(shortcut) = WinitShortcutMapper::from_key_event(event, *modifiers) else {
            return;
        };

        let Some(action) = shortcut_manager.action_for(&shortcut) else {
            return;
        };

        match action {
            ShortcutAction::Reload => state.reload(),
            ShortcutAction::Back => state.go_back(),
            ShortcutAction::Forward => state.go_forward(),
            ShortcutAction::OpenAddressBar => state.begin_address_input(),
        }
    }

    pub(crate) fn handle_address_input(&self, event: &KeyEvent) {
        let Self::Running { state, .. } = self else {
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

    pub(crate) fn update_modifiers(&mut self, new_modifiers: ModifiersState) {
        match self {
            Self::Initial { modifiers, .. } | Self::Running { modifiers, .. } => {
                *modifiers = new_modifiers;
            }
        }
    }
}
