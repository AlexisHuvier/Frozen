use piston_window::*;
use crate::utils::{FPSCounter, Color, Config};
use crate::states::*;

#[derive(Copy, Clone, PartialEq)]
pub enum States {
    Menu,
    Options
}

pub struct App {
    debug: bool,
    menu: Menu,
    options: Options,
    state: States
}

impl App {
    pub fn new(factory: &mut gfx_device_gl::Factory, size: Size) -> App {
        let conf = Config::new("./resources/config.json");
        let debug = conf.get("debug").as_bool().expect("[Config] Debug value must be boolean");
        App {
            debug: debug,
            menu: Menu::new(size),
            options: Options::new(size, conf),
            state: States::Menu
        }
    }

    pub fn run(&mut self, win: &mut PistonWindow) {
        let mut glyphs = win.load_font("./resources/fonts/general.ttf").expect("Unable to load font : general.ttf");
        let mut fps_counter = FPSCounter::new();

            let debug = self.debug;
        while let Some(e) = win.next() {

            match self.state {
                States::Menu => {
                    if let Some(i) = e.press_args() {
                        self.state = self.menu.input(&i, true);
                    }
                    if let Some(i) = e.release_args() {
                        self.state = self.menu.input(&i, false);
                    }
                    self.menu.update();
                },
                States::Options => {
                    if let Some(i) = e.press_args() {
                        self.state = self.options.input(&i, true);
                    }
                    if let Some(i) = e.release_args() {
                        self.state = self.options.input(&i, false);
                    }
                    self.options.update();
                }
            }

            win.draw_2d(&e, |c, g, device| {
                let fps = fps_counter.tick();
                clear(Color::new(0, 197, 255).get_float(), g);
                
                match state {
                    States::Menu => self.menu.draw(c, g, device, &mut glyphs),
                    States::Options => self.options.draw(c, g, device, &mut glyphs),
                }

                if debug {
                    let transform = c.transform.trans(10., 20.);
                    let _res = text::Text::new_color(Color::new(255, 255, 255).get_float(), 20).draw(
                        &*format!("FPS : {}", fps),
                        &mut glyphs,
                        &c.draw_state,
                        transform,
                        g,
                    );
                    glyphs.factory.encoder.flush(device);
                }
            });
        }
    }
}