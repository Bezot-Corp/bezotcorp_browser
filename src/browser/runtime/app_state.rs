use std::cell::RefCell;
use std::sync::Arc;

use tokio::sync::mpsc;
use winit::window::Window;

use crate::browser::{
    chrome::BrowserLayout,
    layout::{LayoutBuilder, Viewport},
    navigation::{AddressInputState, NavigationState},
    network::NetworkResponse,
    render::{BrowserFrame, GpuRenderer, Renderer},
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
    pub(crate) cursor: RefCell<(f32, f32)>,
    pub(crate) scroll_y: RefCell<f32>,
    network_receiver: RefCell<mpsc::Receiver<NetworkResponse>>,
}

impl AppState {
    pub(crate) async fn new(window: Arc<Window>, initial_url: impl Into<String>) -> Self {
        let initial_url = initial_url.into();
        let initial_size = window.inner_size();
        let (network_tx, network_rx) = mpsc::channel(NETWORK_CHANNEL_CAPACITY);

        let gpu_renderer = match GpuRenderer::new(window.clone()).await {
            Ok(renderer) => Some(renderer),
            Err(error) => {
                tracing::error!("GPU renderer init failed: {error}");
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
            cursor: RefCell::new((0.0, 0.0)),
            scroll_y: RefCell::new(0.0),
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

            self.reset_scroll();
            self.sync_address_to_navigation();
            self.window.request_redraw();
        }
    }

    pub(crate) fn render(&self) {
        let viewport = self.content_viewport();
        let toolbar_state = self.toolbar_state();

        let browser_state = self.browser_state.borrow();
        let loading_state = browser_state.loading_state();
        let document = browser_state.engine_state().current_document();

        let layout_tree = LayoutBuilder::build(document);
        let render_tree = Renderer::build_tree(&layout_tree, &viewport);

        drop(browser_state);

        let frame = BrowserFrame::new(&render_tree, &toolbar_state, loading_state);

        if let Some(renderer) = self.gpu_renderer.borrow_mut().as_mut()
            && let Err(error) = renderer.render(frame)
        {
            tracing::error!("render error: {error}");
        }
    }

    pub(crate) fn resize(&self, width: u32, height: u32) {
        if let Some(renderer) = self.gpu_renderer.borrow_mut().as_mut() {
            renderer.resize(width, height);
        }
    }

    pub(crate) fn content_viewport(&self) -> Viewport {
        let size = self.window.inner_size();
        let scroll_y = *self.scroll_y.borrow();

        Viewport::new(0.0, 0.0, size.width as f32, size.height as f32)
            .with_inset(TOOLBAR_HEIGHT, 0.0, 0.0, 0.0)
            .with_scroll(0.0, scroll_y)
    }

    pub(crate) fn scroll_by(&self, dy: f32) {
        let mut scroll_y = self.scroll_y.borrow_mut();
        *scroll_y = (*scroll_y + dy).max(0.0);
    }

    pub(crate) fn reset_scroll(&self) {
        *self.scroll_y.borrow_mut() = 0.0;
    }
}
