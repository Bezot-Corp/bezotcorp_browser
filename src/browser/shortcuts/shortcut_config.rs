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

#[cfg(test)]
mod tests {
    use crate::browser::shortcuts::{Platform, ShortcutAction, ShortcutConfig};

    #[test]
    fn loads_action_first_platform_shortcuts_from_ron() {
        let input = r#"
        {
          Reload: {
            Linux: [
              (key: "F5", modifiers: []),
              (key: "R", modifiers: [Control]),
            ],
          },
        }
        "#;

        let config = ShortcutConfig::from_ron_str(input).expect("config should load");
        let shortcuts = config.shortcuts_for(ShortcutAction::Reload, Platform::Linux);

        assert_eq!(shortcuts.len(), 2);
        assert_eq!(shortcuts[0].key(), "F5");
        assert_eq!(shortcuts[1].key(), "R");
    }
}
