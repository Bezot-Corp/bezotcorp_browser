use std::collections::HashMap;

use crate::browser::shortcuts::{KeyboardShortcut, Platform, ShortcutAction, ShortcutConfig};

pub(crate) struct ShortcutManager {
    platform: Platform,
    actions_by_shortcut: HashMap<KeyboardShortcut, ShortcutAction>,
}

impl ShortcutManager {
    pub(crate) fn from_config(config: &ShortcutConfig, platform: Platform) -> Self {
        let mut actions_by_shortcut = HashMap::new();

        for (action, platforms) in config.bindings() {
            if let Some(shortcuts) = platforms.get(&platform) {
                for shortcut in shortcuts {
                    actions_by_shortcut.insert(shortcut.clone(), *action);
                }
            }
        }

        Self {
            platform,
            actions_by_shortcut,
        }
    }

    pub(crate) fn platform(&self) -> Platform {
        self.platform
    }

    pub(crate) fn action_for(&self, shortcut: &KeyboardShortcut) -> Option<ShortcutAction> {
        self.actions_by_shortcut.get(shortcut).copied()
    }
}
