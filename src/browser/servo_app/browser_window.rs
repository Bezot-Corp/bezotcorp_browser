use std::rc::Rc;

use crate::browser::servo_app::{AppState, ServoBrowserApp};
use crate::browser::shortcuts::{Platform, ShortcutConfig, ShortcutManager};
use euclid::Scale;
use servo::{RenderingContext, ServoBuilder, WebViewBuilder, WindowRenderingContext};
use url::Url;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

impl ServoBrowserApp {
    pub(super) fn create_browser_window(&mut self, event_loop: &ActiveEventLoop) {
        let Self::Initial {
            waker,
            initial_url,
            modifiers,
        } = self
        else {
            return;
        };

        let display_handle = event_loop
            .display_handle()
            .expect("Failed to get display handle");

        let window = Rc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("BezotCorp Browser")
                        .with_inner_size(PhysicalSize::new(1280, 800)),
                )
                .expect("Failed to create window"),
        );

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

        let webview = WebViewBuilder::new(&app_state.servo, app_state.rendering_context.clone())
            .url(url)
            .hidpi_scale_factor(Scale::new(app_state.window.scale_factor() as f32))
            .delegate(app_state.clone())
            .build();

        webview.resize(app_state.content_size());
        app_state.webviews.borrow_mut().push(webview);

        let shortcuts = include_str!("../../../config/keyboard_shortcuts.ron");
        let shortcut_config = ShortcutConfig::from_ron_str(shortcuts)
            .expect("Keyboard shortcut config must be valid");

        let shortcut_manager = ShortcutManager::from_config(&shortcut_config, Platform::current());

        *self = Self::Running {
            state: app_state,
            modifiers: *modifiers,
            shortcut_manager,
        };
    }
}
