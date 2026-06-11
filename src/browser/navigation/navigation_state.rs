use crate::browser::navigation::{NavigationCommand, NavigationEntry};

#[derive(Debug, Clone)]
pub(crate) struct NavigationState {
    current_url: String,
    history: Vec<NavigationEntry>,
    history_index: Option<usize>,
}

impl NavigationState {
    pub(crate) fn new(initial_url: impl Into<String>) -> Self {
        let initial_url = normalize_url(&initial_url.into());
        let entry = NavigationEntry::new(initial_url.clone());

        Self {
            current_url: initial_url,
            history: vec![entry],
            history_index: Some(0),
        }
    }

    pub(crate) fn current_url(&self) -> &str {
        &self.current_url
    }

    pub(crate) fn history(&self) -> &[NavigationEntry] {
        &self.history
    }

    pub(crate) fn can_go_back(&self) -> bool {
        matches!(self.history_index, Some(index) if index > 0)
    }

    pub(crate) fn can_go_forward(&self) -> bool {
        matches!(self.history_index, Some(index) if index + 1 < self.history.len())
    }

    pub(crate) fn navigate_to(&mut self, url: impl Into<String>) -> &str {
        let normalized_url = normalize_url(&url.into());

        if let Some(index) = self.history_index {
            self.history.truncate(index + 1);
        }

        self.history
            .push(NavigationEntry::new(normalized_url.clone()));
        self.history_index = Some(self.history.len() - 1);
        self.current_url = normalized_url;

        self.current_url()
    }

    pub(crate) fn apply_command(&mut self, command: NavigationCommand) -> Option<&str> {
        match command {
            NavigationCommand::Back => self.go_back(),
            NavigationCommand::Forward => self.go_forward(),
            NavigationCommand::Reload => Some(self.current_url()),
        }
    }

    fn go_back(&mut self) -> Option<&str> {
        if !self.can_go_back() {
            return None;
        }

        let next_index = self.history_index? - 1;
        self.history_index = Some(next_index);
        self.current_url = self.history[next_index].url().to_string();

        Some(self.current_url())
    }

    fn go_forward(&mut self) -> Option<&str> {
        if !self.can_go_forward() {
            return None;
        }

        let next_index = self.history_index? + 1;
        self.history_index = Some(next_index);
        self.current_url = self.history[next_index].url().to_string();

        Some(self.current_url())
    }
}

fn normalize_url(input: &str) -> String {
    let trimmed = input.trim();

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_initial_history_entry() {
        let state = NavigationState::new("servo.org");

        assert_eq!(state.current_url(), "https://servo.org");
        assert_eq!(state.history().len(), 1);
    }

    #[test]
    fn navigates_to_new_url() {
        let mut state = NavigationState::new("servo.org");

        state.navigate_to("example.com");

        assert_eq!(state.current_url(), "https://example.com");
        assert_eq!(state.history().len(), 2);
        assert!(state.can_go_back());
    }

    #[test]
    fn goes_back_and_forward() {
        let mut state = NavigationState::new("servo.org");
        state.navigate_to("example.com");

        assert_eq!(
            state.apply_command(NavigationCommand::Back),
            Some("https://servo.org")
        );
        assert_eq!(
            state.apply_command(NavigationCommand::Forward),
            Some("https://example.com")
        );
    }

    #[test]
    fn truncates_forward_history_after_new_navigation() {
        let mut state = NavigationState::new("servo.org");
        state.navigate_to("example.com");
        state.apply_command(NavigationCommand::Back);
        state.navigate_to("mozilla.org");

        assert_eq!(state.current_url(), "https://mozilla.org");
        assert_eq!(state.history().len(), 2);
        assert!(!state.can_go_forward());
    }

    #[test]
    fn reload_keeps_current_url() {
        let mut state = NavigationState::new("servo.org");

        assert_eq!(
            state.apply_command(NavigationCommand::Reload),
            Some("https://servo.org")
        );
    }
}
