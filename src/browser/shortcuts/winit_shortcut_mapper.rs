use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};

use crate::browser::shortcuts::{KeyboardModifier, KeyboardShortcut};

pub(crate) struct WinitShortcutMapper;

impl WinitShortcutMapper {
    pub(crate) fn from_key_event(
        event: &KeyEvent,
        modifiers: ModifiersState,
    ) -> Option<KeyboardShortcut> {
        let key = match event.physical_key {
            PhysicalKey::Code(KeyCode::F5) => "F5",
            PhysicalKey::Code(KeyCode::KeyR) => "R",
            PhysicalKey::Code(KeyCode::ArrowLeft) => "ArrowLeft",
            PhysicalKey::Code(KeyCode::ArrowRight) => "ArrowRight",
            PhysicalKey::Code(KeyCode::KeyL) => "L",
            _ => return None,
        };

        Some(KeyboardShortcut::new(key, Self::map_modifiers(modifiers)))
    }

    fn map_modifiers(modifiers: ModifiersState) -> Vec<KeyboardModifier> {
        let mut mapped = Vec::new();

        if modifiers.control_key() {
            mapped.push(KeyboardModifier::Control);
        }

        if modifiers.alt_key() {
            mapped.push(KeyboardModifier::Alt);
        }

        if modifiers.shift_key() {
            mapped.push(KeyboardModifier::Shift);
        }

        if modifiers.super_key() {
            mapped.push(KeyboardModifier::Super);
        }

        mapped
    }
}
