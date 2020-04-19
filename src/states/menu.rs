use piston_window::*;
use crate::utils::{Position, TextRender, Color};

#[derive(Clone)]
pub struct Menu {
    texts: Vec<TextRender>,
    pub selected_btn: usize,
    win_size: Size
}

impl Menu {
    pub fn new(win_size: Size) -> Menu {
        
        Menu {
            texts: vec![TextRender::new(text::Text::new_color(color::WHITE, 35), "Frozen", Position::new(win_size.width as i32 / 2 - 75, win_size.height as i32 / 2 - 150)),
                TextRender::new(text::Text::new_color(color::WHITE, 25), "Jouer", Position::new(win_size.width as i32 / 2 - 51, win_size.height as i32 / 2 - 50)),
                TextRender::new(text::Text::new_color(color::WHITE, 25), "Options", Position::new(win_size.width as i32 / 2 - 62, win_size.height as i32 / 2)),
                TextRender::new(text::Text::new_color(color::WHITE, 25), "Quitter", Position::new(win_size.width as i32 / 2 - 62, win_size.height as i32 / 2 + 50))],
            selected_btn: 1,
            win_size: win_size
        }
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        let rect1 = [ self.win_size.width / 2. - 105., (self.texts[self.selected_btn].position.y - 30) as f64, 175., 40.];
        let rect2 = [ self.win_size.width / 2. - 103., (self.texts[self.selected_btn].position.y - 28) as f64, 171., 36.];
        rectangle(color::WHITE, rect1, c.transform, g);
        rectangle(Color::new(0, 197, 255).get_float(), rect2, c.transform, g);
        for i in 0..self.texts.len() {
            self.texts[i].draw(c, g, device, glyphs);
        }
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        if is_press {
            if let Button::Keyboard(key) = *button {
                match key {
                    Key::Up => if self.selected_btn > 1 { self.selected_btn -= 1; },
                    Key::Down => if self.selected_btn < 3 { self.selected_btn += 1; },
                    _ => ()
                }
            }
        }
    }
}