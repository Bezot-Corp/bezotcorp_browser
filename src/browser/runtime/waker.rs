use winit::event_loop::{EventLoop, EventLoopProxy};

use crate::browser::runtime::WakerEvent;

#[derive(Clone)]
pub(crate) struct Waker(EventLoopProxy<WakerEvent>);

impl Waker {
    pub(crate) fn new(event_loop: &EventLoop<WakerEvent>) -> Self {
        Self(event_loop.create_proxy())
    }

    pub(crate) fn wake(&self) {
        let _ = self.0.send_event(WakerEvent);
    }
}
