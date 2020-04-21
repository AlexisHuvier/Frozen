use piston_window::*;

use crate::entities::*;
use crate::AppInfo;

pub struct Game {
}

impl Game {
    pub fn new(factory: &mut gfx_device_gl::Factory) -> Game {
        Game {
        }
    }

    pub fn input(&mut self, button: &Button, is_press: bool, info: AppInfo) -> AppInfo { info }

    pub fn update(&mut self, info: AppInfo) -> AppInfo { 
        info 
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
    }
    
}