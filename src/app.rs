use piston_window::*;
use crate::utils::FPSCounter;
use crate::utils::Color;
use crate::states::Menu;

pub enum States {
    Menu
}

pub struct App {
    win: PistonWindow,
    debug: bool,
    menu: Menu,
    state: States 
}

impl App {
    pub fn new(debug: bool) -> App {
        let size = Size { width: 1280., height: 960. };
        App {
            win: WindowSettings::new("Frozen", size).build().unwrap_or_else(|e| panic!("Failed to build App: {}", e)),
            debug: debug,
            menu: Menu::new(size),
            state: States::Menu
        }
    }

    pub fn run(&mut self) {
        let mut glyphs = self.win.load_font("./resources/fonts/general.ttf").expect("Unable to load font : general.ttf");
        let mut fps_counter = FPSCounter::new();

        while let Some(e) = self.win.next() {
            let debug = self.debug;

            if let Some(i) = e.press_args() {
                match self.state {
                    States::Menu => self.menu.input(&i, true)
                }
            }
    
            if let Some(i) = e.release_args() {
                match self.state {
                    States::Menu => self.menu.input(&i, false)
                }
            }

            let mut world;

            match self.state {
                States::Menu => world = self.menu.clone()
            }

            self.win.draw_2d(&e, |c, g, device| {
                let fps = fps_counter.tick();
                clear(Color::new(0, 197, 255).get_float(), g);

                world.draw(c, g, device, &mut glyphs);

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