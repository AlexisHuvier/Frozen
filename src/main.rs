extern crate piston_window;

mod app;
mod utils;
mod states;
mod entities;

pub use app::*;
use piston_window::*;

fn main() {
    let size = Size { width: 1280., height: 960. };
    let mut win: PistonWindow = WindowSettings::new("Frozen", size).resizable(false).build().unwrap_or_else(|e| panic!("Failed to build App: {}", e));
    let mut factory = win.factory.clone();
    let mut app = App::new(&mut factory, size);
    app.run(&mut win, &mut factory);
}
