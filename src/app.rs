use piston_window::*;
use crate::utils::{FPSCounter, Color, Config};
use crate::states::*;

#[derive(Copy, Clone, PartialEq)]
pub enum States {
    Menu,
    Options,
    Game
}

#[derive(Copy, Clone, PartialEq)]
pub struct AppInfo {
    pub debug: bool,
    pub state: States
}

impl AppInfo {
    pub fn new(conf: &Config) -> AppInfo {
        AppInfo {
            debug: conf.get("debug").as_bool().expect("[Config] Debug value must be boolean"),
            state: States::Menu
        } 
    }
}

pub struct App {
    info: AppInfo,
    menu: Menu,
    game: Game,
    options: Options,
}

impl App {
    pub fn new(factory: &mut gfx_device_gl::Factory, size: Size) -> App {
        let conf = Config::new("./resources/config.json");
        App {
            info: AppInfo::new(&conf),
            menu: Menu::new(size),
            game: Game::new(factory),
            options: Options::new(size, conf)
        }
    }

    pub fn run(&mut self, win: &mut PistonWindow) {
        let mut glyphs = win.load_font("./resources/fonts/general.ttf").expect("Unable to load font : general.ttf");
        let mut fps_counter = FPSCounter::new();

        while let Some(e) = win.next() {

            match self.info.state {
                States::Menu => {
                    if let Some(i) = e.press_args() {
                        self.info = self.menu.input(&i, true, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.menu.input(&i, false, self.info);
                    }
                    self.info = self.menu.update(self.info);
                },
                States::Options => {
                    if let Some(i) = e.press_args() {
                        self.info = self.options.input(&i, true, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.options.input(&i, false, self.info);
                    }
                    self.info = self.options.update(self.info);
                },
                States::Game => {
                    if let Some(i) = e.press_args() {
                        self.info = self.game.input(&i, true, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.game.input(&i, false, self.info);
                    }
                    self.info = self.game.update(self.info);
                }
            }

            win.draw_2d(&e, |c, g, device| {
                let fps = fps_counter.tick();
                clear(Color::new(0, 197, 255).get_float(), g);
                
                match self.info.state {
                    States::Menu => self.menu.draw(c, g, device, &mut glyphs),
                    States::Options => self.options.draw(c, g, device, &mut glyphs),
                    States::Game => self.game.draw(c, g, device, &mut glyphs)
                }

                if self.info.debug {
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