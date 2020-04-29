use piston_window::*;
use crate::utils::{Position, TextRender};
use crate::{AppInfo, States};

#[derive(Clone)]
pub struct Win {
    texts: Vec<TextRender>,
    win_size: Size
}

impl Win {
    pub fn new(win_size: Size) -> Win {
        
        Win {
            texts: vec![TextRender::new(text::Text::new_color(color::WHITE, 35), "Bravo !", Position::new(win_size.width as i32 / 2 - 75, win_size.height as i32 / 2 - 150)),
                TextRender::new(text::Text::new_color(color::WHITE, 25), "Merci d'avoir jouer au jeu.", Position::new(win_size.width as i32 / 2 - 150, win_size.height as i32 / 2))],
            win_size: win_size
        }
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        for i in 0..self.texts.len() {
            self.texts[i].draw(c, g, device, glyphs);
        }
    }

    pub fn input(&mut self, button: &Button, is_press: bool, info: AppInfo) -> AppInfo {
        let mut info = info;
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Return => info.state = States::Menu,
                    _ => ()
                }
            }
        }
        info
    }

    pub fn update(&mut self, info: AppInfo) -> AppInfo { info }
}