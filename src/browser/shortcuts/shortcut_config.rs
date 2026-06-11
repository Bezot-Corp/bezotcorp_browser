use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::browser::shortcuts::{KeyboardShortcut, Platform, ShortcutAction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ShortcutConfig {
    bindings: HashMap<ShortcutAction, HashMap<Platform, Vec<KeyboardShortcut>>>,
}

impl ShortcutConfig {
    pub(crate) fn from_ron_str(input: &str) -> Result<Self, ron::Error> {
        let bindings = ron::from_str(input)?;

        Ok(Self { bindings })
    }

    pub(crate) fn shortcuts_for(
        &self,
        action: ShortcutAction,
        platform: Platform,
    ) -> &[KeyboardShortcut] {
        self.bindings
            .get(&action)
            .and_then(|platforms| platforms.get(&platform))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub(crate) fn bindings(
        &self,
    ) -> &HashMap<ShortcutAction, HashMap<Platform, Vec<KeyboardShortcut>>> {
        &self.bindings
    }
}
