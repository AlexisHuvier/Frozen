use piston_window::*;

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
            platforms: vec!(Platform::new(Position::new(100, 500), "./resources/images/Tiles/15.png", factory))
        }
    }

    pub fn input(&mut self, button: &Button, is_press: bool, info: AppInfo) -> AppInfo { info }

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