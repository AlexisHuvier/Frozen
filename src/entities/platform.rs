use sprite::*;
use piston_window::*;
use crate::utils::{sprite::load_sprite, Position};

pub struct Platform {
    pub pos: Position,
    pub sprite: Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>
}

impl Platform {
    pub fn new(pos: Position, sprite: &str, factory: &mut gfx_device_gl::Factory) -> Platform {
        let mut sprite = load_sprite(factory, sprite);
        sprite.set_scale(0.59375, 0.59375);
        sprite.set_position(pos.x as f64, pos.y as f64);
        Platform {
            pos: pos,
            sprite: sprite
        }
    }

    pub fn render<G>(&mut self, c: piston_window::Context, g: &mut G) 
        where G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>
    {
        self.sprite.draw(c.transform, g);
    }
}