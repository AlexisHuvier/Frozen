extern crate piston_window;

mod app;
mod utils;
mod states;

pub use app::*;

fn main() {
    let mut app = App::new(true);
    app.run();
}
