use std::cell::RefCell;
use std::sync::Arc;

use tokio::sync::mpsc;
use winit::window::Window;

use crate::browser::{
    chrome::BrowserLayout,
    layout::Viewport,
    navigation::{AddressInputState, NavigationState},
    network::NetworkResponse,
    render::GpuRenderer,
    state::{BrowserLoadingState, BrowserState},
};

const TOOLBAR_HEIGHT: f32 = 72.0;
const NETWORK_CHANNEL_CAPACITY: usize = 64;

pub(crate) struct AppState {
    pub(crate) window: Arc<Window>,
    pub(crate) navigation: RefCell<NavigationState>,
    pub(crate) address_input: RefCell<AddressInputState>,
    pub(crate) browser_state: RefCell<BrowserState>,
    pub(crate) layout: RefCell<BrowserLayout>,
    pub(crate) gpu_renderer: RefCell<Option<GpuRenderer>>,
    network_receiver: RefCell<mpsc::Receiver<NetworkResponse>>,
}

impl AppState {
    pub(crate) async fn new(window: Arc<Window>, initial_url: impl Into<String>) -> Self {
        let initial_url = initial_url.into();
        let initial_size = window.inner_size();
        let (network_tx, network_rx) = mpsc::channel(NETWORK_CHANNEL_CAPACITY);

        let gpu_renderer = match GpuRenderer::new(window.clone()).await {
            Ok(renderer) => Some(renderer),
            Err(e) => {
                tracing::error!("GPU renderer init failed: {e}");
                None
            }
        };

        Self {
            window,
            navigation: RefCell::new(NavigationState::new(initial_url.clone())),
            address_input: RefCell::new(AddressInputState::default()),
            browser_state: RefCell::new(BrowserState::new(initial_url, network_tx)),
            layout: RefCell::new(BrowserLayout::new(initial_size.width, initial_size.height)),
            gpu_renderer: RefCell::new(gpu_renderer),
            network_receiver: RefCell::new(network_rx),
        }
    }

    pub(crate) fn poll_network(&self) {
        let mut receiver = self.network_receiver.borrow_mut();
        while let Ok(response) = receiver.try_recv() {
            let mut browser_state = self.browser_state.borrow_mut();
            browser_state
                .engine_state_mut()
                .bezot_engine_mut()
                .apply_response(response);
            browser_state.set_loading_state(BrowserLoadingState::Idle);
            drop(browser_state);
            self.window.request_redraw();
        }
    }

    pub(crate) fn render(&self) {
        let viewport = self.content_viewport();
        let toolbar_state = self.toolbar_state();
        let browser_state = self.browser_state.borrow();
        let loading_state = browser_state.loading_state();
        let render_tree = browser_state.engine_state().bezot_render_tree(&viewport);
        drop(browser_state);

        if let Some(renderer) = self.gpu_renderer.borrow_mut().as_mut() {
            if let Err(e) = renderer.render(&render_tree, &toolbar_state, loading_state) {
                tracing::error!("render error: {e}");
            }
        }
    }

    pub(crate) fn resize(&self, width: u32, height: u32) {
        if let Some(renderer) = self.gpu_renderer.borrow_mut().as_mut() {
            renderer.resize(width, height);
        }
    }

    pub(crate) fn content_viewport(&self) -> Viewport {
        let size = self.window.inner_size();
        Viewport::new(0.0, 0.0, size.width as f32, size.height as f32).with_inset(
            TOOLBAR_HEIGHT,
            0.0,
            0.0,
            0.0,
        )
    }
}
