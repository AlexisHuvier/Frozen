use piston_window::*;
use crate::utils::{FPSCounter, Color, Config};
use crate::states::*;

#[derive(Copy, Clone, PartialEq)]
pub enum States {
    Menu,
    Options,
    Game,
    Win
}

#[derive(Copy, Clone, PartialEq)]
pub struct AppInfo {
    pub debug: bool,
    pub unlimited_icebox: bool,
    pub state: States
}

impl AppInfo {
    pub fn new(conf: &Config) -> AppInfo {
        AppInfo {
            debug: conf.get("debug").as_bool().expect("[Config] Debug value must be boolean"),
            unlimited_icebox: conf.get("unlimited_icebox").as_bool().expect("[Config] Unlimited Icebox value must be boolean"),
            state: States::Menu
        } 
    }
}

pub struct App {
    info: AppInfo,
    menu: Menu,
    game: Game,
    options: Options,
    win: Win
}

impl App {
    pub fn new(factory: &mut gfx_device_gl::Factory, size: Size) -> App {
        let conf = Config::new("./resources/config.json");
        let mut game = Game::new(factory);
        game.level(1, factory);
        App {
            info: AppInfo::new(&conf),
            menu: Menu::new(size),
            game: game,
            options: Options::new(size, conf),
            win: Win::new(size)
        }
    }

    pub fn run(&mut self, win: &mut PistonWindow, factory: &mut gfx_device_gl::Factory) {
        let mut glyphs = win.load_font("./resources/fonts/general.ttf").expect("Unable to load font : general.ttf");
        let mut fps_counter = FPSCounter::new();

        while let Some(e) = win.next() {

            match self.info.state {
                States::Menu => {
                    if let Some(i) = e.press_args() {
                        let info = self.menu.input(&i, true, self.info);
                        if info.state == States::Game {
                            self.game.level(1, factory);
                        }
                        self.info = info;
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.menu.input(&i, false, self.info);
                    }
                },
                States::Options => {
                    if let Some(i) = e.press_args() {
                        self.info = self.options.input(&i, true, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.options.input(&i, false, self.info);
                    }
                },
                States::Game => {
                    if let Some(i) = e.press_args() {
                        self.info = self.game.input(&i, true, factory, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.game.input(&i, false, factory, self.info);
                    }
                    e.mouse_cursor(|pos| {
                        self.game.mouse_move(pos);
                    });
                },
                States::Win => {
                    if let Some(i) = e.press_args() {
                        self.info = self.win.input(&i, true, self.info);
                    }
                    if let Some(i) = e.release_args() {
                        self.info = self.win.input(&i, false, self.info);
                    }
                }
            }

            win.draw_2d(&e, |c, g, device| {
                let fps = fps_counter.tick();
                clear(Color::new(0, 197, 255).get_float(), g);
                
                match self.info.state {
                    States::Menu => {
                        self.menu.draw(c, g, device, &mut glyphs);
                        self.info = self.menu.update(self.info);
                    },
                    States::Options => {
                        self.options.draw(c, g, device, &mut glyphs);
                        self.info = self.options.update(self.info);
                    },
                    States::Game => {
                        self.game.draw(c, g, device, &mut glyphs);
                        self.info = self.game.update(factory, self.info);
                    },
                    States::Win => {
                        self.win.draw(c, g, device, &mut glyphs);
                        self.info = self.win.update(self.info);
                    }
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