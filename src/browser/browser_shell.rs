use std::error;

use tao::dpi::LogicalSize;
use tao::event::{ElementState, Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::WindowBuilder;
use wry::WebViewBuilder;

use crate::browser::browser_state::BrowserState;
use crate::browser::browser_toolbar::BrowserToolbar;

pub struct BrowserShell;

impl BrowserShell {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self) -> Result<(), Box<dyn error::Error>> {
        let event_loop = EventLoop::new();
        let state = BrowserState::new();

        let window = WindowBuilder::new()
            .with_title("BezotCorp Browser")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .build(&event_loop)?;

        let webview = WebViewBuilder::new()
            .with_url(state.current_url())
            .build(&window)?;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }

                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key: KeyCode::KeyL,
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    let script = BrowserToolbar::navigation_prompt_script(state.current_url());
                    let _ = webview.evaluate_script(&script);
                }

                _ => {}
            }
        });
    }
}
