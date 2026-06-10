use crate::app::App;

mod ai;
mod app;
mod bridge;
mod browser;
mod conversations;
mod storage;
mod ui;

fn main() {
    let app = App::new();
    app.run();
}
