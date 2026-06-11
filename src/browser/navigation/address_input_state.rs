#[derive(Debug, Default, Clone)]
pub(crate) struct AddressInputState {
    active: bool,
    buffer: String,
}

impl AddressInputState {
    pub(crate) fn is_active(&self) -> bool {
        self.active
    }

    pub(crate) fn activate(&mut self, current_url: &str) {
        self.active = true;
        self.buffer.clear();
        self.buffer.push_str(current_url);
    }

    pub(crate) fn deactivate(&mut self) {
        self.active = false;
    }

    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }

    pub(crate) fn push(&mut self, c: char) {
        self.buffer.push(c);
    }

    pub(crate) fn pop(&mut self) {
        self.buffer.pop();
    }

    pub(crate) fn value(&self) -> &str {
        &self.buffer
    }

    pub(crate) fn replace(&mut self, value: impl Into<String>) {
        self.buffer = value.into();
    }

    pub(crate) fn append_char(&mut self, character: char) {
        self.buffer.push(character);
    }

    pub(crate) fn remove_last_char(&mut self) {
        self.buffer.pop();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
