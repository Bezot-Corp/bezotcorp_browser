use crate::browser::BrowserShell;

pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        BrowserShell::new().run()
    }
}
