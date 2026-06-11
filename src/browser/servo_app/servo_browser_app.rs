use std::rc::Rc;

use euclid::Scale;
use servo::{
    InputEvent, RenderingContext, ServoBuilder, WebViewBuilder, WheelDelta, WheelEvent, WheelMode,
    WindowRenderingContext,
};
use url::Url;
use webrender_api::units::DevicePoint;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowId};

use crate::browser::servo_app::{AppState, Waker, WakerEvent};

pub(crate) enum ServoBrowserApp {
    Initial { waker: Waker, initial_url: String },
    Running(Rc<AppState>),
}

impl ServoBrowserApp {
    pub(crate) fn new(event_loop: &EventLoop<WakerEvent>, initial_url: impl Into<String>) -> Self {
        Self::Initial {
            waker: Waker::new(event_loop),
            initial_url: initial_url.into(),
        }
    }
}

impl ApplicationHandler<WakerEvent> for ServoBrowserApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Self::Initial { waker, initial_url } = self {
            let display_handle = event_loop
                .display_handle()
                .expect("Failed to get display handle");

            let window = event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("BezotCorp Browser")
                        .with_inner_size(PhysicalSize::new(1280, 800)),
                )
                .expect("Failed to create window");

            let window_handle = window.window_handle().expect("Failed to get window handle");

            let rendering_context = Rc::new(
                WindowRenderingContext::new(display_handle, window_handle, window.inner_size())
                    .expect("Could not create Servo rendering context"),
            );

            let _ = rendering_context.make_current();

            let servo = ServoBuilder::default()
                .event_loop_waker(Box::new(waker.clone()))
                .build();

            servo.setup_logging();

            let app_state = Rc::new(AppState::new(
                window,
                servo,
                rendering_context,
                initial_url.clone(),
            ));

            let url = Url::parse(initial_url).expect("Initial URL must be valid");

            let webview =
                WebViewBuilder::new(&app_state.servo, app_state.rendering_context.clone())
                    .url(url)
                    .hidpi_scale_factor(Scale::new(app_state.window.scale_factor() as f32))
                    .delegate(app_state.clone())
                    .build();

            app_state.webviews.borrow_mut().push(webview);

            *self = Self::Running(app_state);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: WakerEvent) {
        if let Self::Running(state) = self {
            state.servo.spin_event_loop();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Self::Running(state) = self {
            state.servo.spin_event_loop();
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Self::Running(state) = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    webview.paint();
                    state.rendering_context.present();
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Self::Running(state) = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    let (delta_x, delta_y, mode) = match delta {
                        MouseScrollDelta::LineDelta(dx, dy) => {
                            ((dx * 76.0) as f64, (dy * 76.0) as f64, WheelMode::DeltaLine)
                        }
                        MouseScrollDelta::PixelDelta(delta) => {
                            (delta.x, delta.y, WheelMode::DeltaPixel)
                        }
                    };

                    webview.notify_input_event(InputEvent::Wheel(WheelEvent::new(
                        WheelDelta {
                            x: delta_x,
                            y: delta_y,
                            z: 0.0,
                            mode,
                        },
                        DevicePoint::default().into(),
                    )));
                }
            }
            WindowEvent::Resized(new_size) => {
                if let Self::Running(state) = self
                    && let Some(webview) = state.webviews.borrow().last()
                {
                    webview.resize(new_size);
                }
            }
            _ => {}
        }
    }
}
