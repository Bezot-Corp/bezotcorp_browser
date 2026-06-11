use serde::{Deserialize, Serialize};

use crate::browser::shortcuts::KeyboardModifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct KeyboardShortcut {
    key: String,
    modifiers: Vec<KeyboardModifier>,
}

impl KeyboardShortcut {
    pub(crate) fn new(key: impl Into<String>, modifiers: Vec<KeyboardModifier>) -> Self {
        Self {
            key: key.into(),
            modifiers,
        }
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn modifiers(&self) -> &[KeyboardModifier] {
        &self.modifiers
    }
}
