use piston_window::*;
use std::fs::File;
use std::io::Read;

use crate::entities::*;
use crate::utils::Position;
use crate::AppInfo;

pub struct Game {
    pub elsa: Elsa,
    pub platforms: Vec<Platform>
}

impl Game {
    pub fn new(factory: &mut gfx_device_gl::Factory) -> Game {
        Game {
            elsa: Elsa::new(factory),
            platforms: vec!()
        }
    }

    pub fn level(&mut self, nb: i64, factory: &mut gfx_device_gl::Factory) {
        let mut file = File::open(&*format!("./resources/levels/{}.json", nb)).expect(&*format!("[Level] Level {} non trouvée", nb));
        let mut data = String::new();
        let _res = file.read_to_string(&mut data);
        let json_data = json::parse(&*data).expect(&*format!("[Level] Level {} mal formée", nb));

        let player = &json_data["player"];
        let map = &json_data["map"];

        self.elsa.pos = Position::new(player["x"].as_i32().expect("[Level] Player X Pos must be a integer."), player["y"].as_i32().expect("[Level] Player Y Pos must be a integer."));

        let mappos = Position::new(map["x"].as_i32().expect("[Level] Map X Pos must be a integer."), map["y"].as_i32().expect("[Level] Map Y Pos must be a integer."));
        let mut platforms: Vec<Platform> = vec!();
        
        for x in 0..map["blocks"].len() {
            for y in 0..map["blocks"][x].len() {
                let id = map["blocks"][x][y].as_i32().expect(&*format!("[Level] Block ({}, {}) must be a integer.", y, x));
                if id != 0 {
                    let plat_pos = Position::new(y as i32*76+mappos.x, x as i32*76+mappos.y);
                    platforms.push(Platform::new(plat_pos, &*format!("./resources/images/Tiles/{}.png", id), factory));
                }
            }
        }

        self.platforms = platforms;
    }

    pub fn input(&mut self, button: &Button, is_press: bool, info: AppInfo) -> AppInfo { 
        if let Button::Keyboard(key) = *button {
            self.elsa.input(key, is_press);
        }
        info 
    }

    pub fn update(&mut self, info: AppInfo) -> AppInfo { 
        self.elsa.update(&self.platforms);
        info 
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        self.elsa.render(c, g);
        for i in 0..self.platforms.len() {
            self.platforms[i].render(c, g);
        }
    }
    
}