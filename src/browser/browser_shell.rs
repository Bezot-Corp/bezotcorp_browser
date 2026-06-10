use std::error;

use crate::browser::engine::{BrowserEngine, ServoEngine};

const INITIAL_URL: &str = "https://servo.org";

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        let mut engine = ServoEngine::new(INITIAL_URL);

        println!("BezotCorp Browser");
        println!("Engine: {}", engine.name());
        println!("Initial URL: {}", engine.current_url());

        engine.load_url(INITIAL_URL);

        Ok(())
    }
}
