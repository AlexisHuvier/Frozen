use piston_window::*;
use sprite::*;
use std::fs::File;
use std::io::Read;

use crate::entities::*;
use crate::utils::{Position, sprite::load_sprite, TextRender};
use crate::{AppInfo, States};

pub struct Game {
    pub elsa: Elsa,
    pub platforms: Vec<Platform>,
    pub mouse_pos: [f64; 2],
    pub nb_ice: u8,
    unlimited_icebox: bool,
    pub lvl: u8,
    pub icon_icebox: Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>,
    pub text_icebox: TextRender,
    pub text_restart: TextRender,
    pub text_level: TextRender,
    pub bg: Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>,
    pub win: Platform
}

impl Game {
    pub fn new(factory: &mut gfx_device_gl::Factory) -> Game {
        let mut sprite = load_sprite(factory, "./resources/images/Objects/IceBox.png");
        sprite.set_scale(0.5, 0.5);
        sprite.set_position(1150., 40.);
        let mut bg = load_sprite(factory, "./resources/images/BG/BG.png");
        bg.set_scale(1.1, 1.1);
        bg.set_position(640., 480.);
        Game {
            elsa: Elsa::new(factory),
            platforms: vec!(),
            mouse_pos: [0., 0.],
            nb_ice: 0,
            unlimited_icebox: false,
            lvl: 0,
            icon_icebox: sprite,
            text_icebox: TextRender::new(text::Text::new_color(color::WHITE, 30), "0", Position::new(1200, 50)),
            text_restart: TextRender::new(text::Text::new_color(color::WHITE, 30), "R to Restart", Position::new(520, 100)),
            text_level: TextRender::new(text::Text::new_color(color::WHITE, 30), "Niveau 0", Position::new(550, 50)),
            bg: bg,
            win: Platform::new(Position::new(0, 0), "./resources/images/Objects/Sign_2.png", factory, 1.)
        }
    }

    pub fn level(&mut self, nb: u8, factory: &mut gfx_device_gl::Factory) {
        let mut file = File::open(&*format!("./resources/levels/{}.json", nb)).expect(&*format!("[Level] Level {} non trouvée", nb));
        let mut data = String::new();
        let _res = file.read_to_string(&mut data);
        let json_data = json::parse(&*data).expect(&*format!("[Level] Level {} mal formée", nb));

        let player = &json_data["player"];
        let map = &json_data["map"];

        self.nb_ice = json_data["ice"].as_u8().expect("[Level] Number of Ice must be a unsigned integer.");
        self.text_icebox.text = self.nb_ice.to_string();

        self.elsa.pos = Position::new(player["x"].as_i32().expect("[Level] Player X Pos must be a integer."), player["y"].as_i32().expect("[Level] Player Y Pos must be a integer."));
        self.elsa.movements = [false, false, false];
        self.elsa.grounded = false;
        self.elsa.gravity = 5;

        let mappos = Position::new(map["x"].as_i32().expect("[Level] Map X Pos must be a integer."), map["y"].as_i32().expect("[Level] Map Y Pos must be a integer."));
        let mut platforms: Vec<Platform> = vec!();
        
        for x in 0..map["blocks"].len() {
            for y in 0..map["blocks"][x].len() {
                let id = map["blocks"][x][y].as_i32().expect(&*format!("[Level] Block ({}, {}) must be a integer.", y, x));
                if id == 4 {
                    self.win.set_position(Position::new(y as i32*76+mappos.x, x as i32*76+mappos.y));
                }
                else if id != 0 {
                    let plat_pos = Position::new(y as i32*76+mappos.x, x as i32*76+mappos.y);
                    platforms.push(Platform::new(plat_pos, &*format!("./resources/images/Tiles/{}.png", id), factory, 0.59375));
                }
            }
        }

        self.platforms = platforms;
        self.lvl = nb;
        self.text_level.text = format!("Niveau {}", nb);
    }

    pub fn mouse_move(&mut self, pos: [f64; 2]) {
        self.mouse_pos = pos;
    }

    pub fn input(&mut self, button: &Button, is_press: bool, factory: &mut gfx_device_gl::Factory, info: AppInfo) -> AppInfo { 
        let mut info = info;
        if let Button::Keyboard(key) = *button {
            self.elsa.input(key, is_press);
            if is_press {
                match key {
                    Key::R => self.level(self.lvl, factory),
                    Key::Escape => info.state = States::Menu,
                    _ => ()
                }
            }
        }
        if let Button::Mouse(btn) = *button {
            if is_press {
                match btn {
                    MouseButton::Left => {
                        if self.nb_ice > 0 {
                            self.platforms.push(Platform::new(Position::new(self.mouse_pos[0] as i32, self.mouse_pos[1] as i32), "./resources/images/Objects/IceBox.png", factory, 0.59375));
                            if !self.unlimited_icebox {
                                self.nb_ice -= 1;
                            }
                            self.text_icebox.text = self.nb_ice.to_string();
                        }
                    },
                    _ => ()
                }
            }
        }
        info 
    }

    pub fn update(&mut self, factory: &mut gfx_device_gl::Factory, info: AppInfo) -> AppInfo { 
        let mut info = info;
        self.unlimited_icebox = info.unlimited_icebox;
        if self.elsa.update(&self.platforms, &self.win) {
            if self.lvl == 2 {
                info.state = States::Win;
            }
            else {
                self.level(self.lvl + 1, factory);
            }
        };
        if self.elsa.pos.y > 1000 {
            self.level(self.lvl, factory)
        }
        info 
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        self.bg.draw(c.transform, g);
        self.elsa.render(c, g);
        for i in 0..self.platforms.len() {
            self.platforms[i].render(c, g);
        }
        self.win.render(c, g);
        self.icon_icebox.draw(c.transform, g);
        self.text_level.draw(c, g, device, glyphs);
        self.text_icebox.draw(c, g, device, glyphs);
        if self.nb_ice == 0 {
            self.text_restart.draw(c, g, device, glyphs);
        }
    }
    
}