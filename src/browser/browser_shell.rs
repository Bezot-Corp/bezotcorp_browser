use std::error::Error;
use std::sync::Arc;

use crate::browser::runtime::BrowserApp;

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        rustls::crypto::aws_lc_rs::default_provider()
            .install_default()
            .expect("Failed to install crypto provider");

        let tokio_runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()?,
        );

        let event_loop = winit::event_loop::EventLoop::new()?;
        let mut app = BrowserApp::new(tokio_runtime);
        Ok(event_loop.run_app(&mut app)?)
    }
}
