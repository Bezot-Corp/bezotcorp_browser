use crate::browser::engine::{BrowserEngine, ServoEngine};

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = ServoEngine::new();

        println!("BezotCorp Browser");
        println!("Rendering engine: {}", engine.name());
        println!("Initial URL: {}", engine.current_url());

        engine.load_url("https://servo.org");

        println!("Loaded URL: {}", engine.current_url());

        Ok(())
    }
}
