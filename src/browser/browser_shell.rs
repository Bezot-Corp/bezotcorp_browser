use std::error::Error;

use crate::browser::runtime::ServoBrowserApp;

const INITIAL_URL: &str = "https://servo.org";

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install crypto provider");

        let event_loop = winit::event_loop::EventLoop::with_user_event()
            .build()
            .expect("Failed to create event loop");

        let mut app = ServoBrowserApp::new(&event_loop, INITIAL_URL);

        Ok(event_loop.run_app(&mut app)?)
    }
}
