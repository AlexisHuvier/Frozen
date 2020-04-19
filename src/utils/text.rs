use piston_window::*;
use crate::utils::Position;

#[derive(Clone)]
pub struct TextRender {
    font: text::Text,
    pub text: String,
    pub position: Position
}

impl TextRender {
    pub fn new(font: text::Text, text: &str, position: Position) -> TextRender {
        TextRender {
            font: font,
            text: text.to_owned(),
            position: position
        }
    }

    pub fn draw<G>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) 
        where G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>
    {
        let transform = c.transform.trans(self.position.x as f64, self.position.y as f64);
        let _res = self.font.draw(&*self.text, glyphs, &c.draw_state, transform, g).unwrap();
        
        glyphs.factory.encoder.flush(device);
    }
}
