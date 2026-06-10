mod ai;
mod application;
mod bridge;
mod browser;
mod conversations;
mod storage;
mod ui;

use application::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    app.run()
}
