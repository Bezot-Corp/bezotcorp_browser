use crate::browser::servo_app::ServoBrowserApp;

impl ServoBrowserApp {
    pub(crate) fn spin_servo(&self) {
        if let Self::Running { state, .. } = self {
            state.servo.spin_event_loop();
        }
    }
}
