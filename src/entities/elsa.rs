use sprite::*;
use piston_window::*;

use crate::utils::{sprite::load_sprite, Position};

pub enum ElsaAnimations {
    IDLE
}

pub struct Elsa {
    pub pos: Position,
    pub idle: Vec<Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>>,
    pub sprite: usize,
    pub anim: ElsaAnimations
}

impl Elsa {
    pub fn new(factory: &mut gfx_device_gl::Factory) -> Elsa {
        let mut idle = vec![
            load_sprite(factory, "./resources/images/Elsa/IDLE (1).png")
        ];

        for i in 0..idle.len() {
            idle[i].set_scale(0.25, 0.25);
        }

        Elsa {
            pos: Position::new(100, 100),
            idle: idle,
            sprite: 0,
            anim: ElsaAnimations::IDLE
        }
    }

    pub fn update_sprite(&mut self, change_pos: bool) {
        if change_pos {
            match self.anim {
                ElsaAnimations::IDLE => self.idle[self.sprite].set_position(self.pos.x as f64, self.pos.y as f64)
            }
        }
    }

    pub fn render<G>(&mut self, c: piston_window::Context, g: &mut G) 
        where G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>
    {
        match self.anim {
            ElsaAnimations::IDLE => self.idle[self.sprite].draw(c.transform, g)
        }
    }
}