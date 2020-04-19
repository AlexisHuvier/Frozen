use piston_window::*;
use crate::utils::{Position, TextRender};
use crate::States;

#[derive(Clone)]
pub struct Options {
    texts: Vec<TextRender>,
    win_size: Size
}

impl Options {
    pub fn new(win_size: Size) -> Options {
        Options {
            texts: vec![TextRender::new(text::Text::new_color(color::WHITE, 35), "Options", Position::new(win_size.width as i32 / 2 - 80, win_size.height as i32 / 2 - 150))],
            win_size: win_size
        }
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        for i in 0..self.texts.len() {
            self.texts[i].draw(c, g, device, glyphs);
        }
    }

    pub fn input(&mut self, button: &Button, is_press: bool) -> States {
        States::Options
    }

    pub fn update(&mut self) { }
}