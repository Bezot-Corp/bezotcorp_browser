use tracing::warn;
use winit::event_loop::{EventLoop, EventLoopProxy};

use crate::browser::runtime::WakerEvent;

#[derive(Clone)]
pub(crate) struct Waker(EventLoopProxy<WakerEvent>);

impl Waker {
    pub(crate) fn new(event_loop: &EventLoop<WakerEvent>) -> Self {
        Self(event_loop.create_proxy())
    }
}

impl servo::EventLoopWaker for Waker {
    fn clone_box(&self) -> Box<dyn servo::EventLoopWaker> {
        Box::new(Self(self.0.clone()))
    }

    fn wake(&self) {
        if let Err(error) = self.0.send_event(WakerEvent) {
            warn!(?error, "Failed to wake Servo event loop");
        }
    }
}
