#[derive(Debug, Default, Clone)]
pub(crate) struct AddressInputState {
    active: bool,
    buffer: String,
    cursor: usize,
}

impl AddressInputState {
    pub(crate) fn is_active(&self) -> bool {
        self.active
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.buffer.len()
    }

    pub(crate) fn value(&self) -> &str {
        &self.buffer
    }

    pub(crate) fn cursor_position(&self) -> usize {
        self.cursor
    }

    pub(crate) fn activate(&mut self, current_url: &str) {
        self.active = true;
        self.buffer.clear();
        self.buffer.push_str(current_url);
        self.cursor = self.buffer.len();
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
        self.cursor = 0;
    }

    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    pub(crate) fn set_value(&mut self, value: impl Into<String>) {
        self.buffer = value.into();
        self.cursor = self.buffer.len();
    }

    pub(crate) fn append_char(&mut self, character: char) {
        self.buffer.push(character);
        self.cursor = self.buffer.len();
    }

    pub(crate) fn remove_last_char(&mut self) {
        self.buffer.pop();
        self.cursor = self.buffer.len();
    }

    pub(crate) fn push(&mut self, c: char) {
        self.append_char(c);
    }

    pub(crate) fn pop(&mut self) {
        self.remove_last_char();
    }

    pub(crate) fn replace(&mut self, value: impl Into<String>) {
        self.set_value(value);
    }
}
