use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

const INITIAL_URL: &str = "https://www.google.com";

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("BezotCorp Browser")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .build(&event_loop)?;

        let _webview = WebViewBuilder::new().with_url(INITIAL_URL).build(&window)?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
