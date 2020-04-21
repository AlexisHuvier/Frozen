use sprite::*;
use piston_window::*;

use crate::entities::Platform;
use crate::utils::{sprite::load_sprite, Position};

pub enum ElsaAnimations {
    IDLE
}

pub struct Elsa {
    pub pos: Position,
    pub idle: Vec<Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>>,
    pub sprite: usize,
    max_gravity: i8,
    pub gravity: i8,
    time_gravity: u8,
    pub grounded: bool,
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
            max_gravity: 5,
            gravity: 5,
            time_gravity: 5,
            grounded: false,
            anim: ElsaAnimations::IDLE
        }
    }

    pub fn get_current_sprite(&mut self) -> &mut Sprite<gfx_texture::Texture<gfx_device_gl::Resources>> {
        match self.anim {
            ElsaAnimations::IDLE => return &mut self.idle[self.sprite]
        }
    }

    pub fn update_sprite(&mut self, change_pos: bool) {
        if change_pos {
            let x = self.pos.x;
            let y = self.pos.y;
            self.get_current_sprite().set_position(x as f64, y as f64);
        }
    }

    pub fn can_go(&mut self, position: &Position, platforms: &Vec<Platform>) -> bool {
        let elsa_bounding = self.get_current_sprite().bounding_box();
        let px = position.x as f64 - elsa_bounding[2] / 2.;
        let py = position.y as f64 - elsa_bounding[3] / 2.;

        for i in 0..platforms.len() {
            let pls = platforms[i].sprite.bounding_box();
            let plx = pls[0];
            let ply = pls[1];
            if px < plx + pls[2] && px + elsa_bounding[2] > plx && py < ply + pls[3] && py + elsa_bounding[3] > ply {
                return false;
            }
        }

        true
    }

    pub fn update(&mut self, platforms: &Vec<Platform>) {
        //Update Gravity
        let futurpos = Position::new(self.pos.x, self.pos.y + self.gravity as i32);
        if self.can_go(&futurpos, platforms) {
            self.grounded = false;
            self.pos = futurpos;
        }
        else if self.gravity > 0 {
            self.grounded = true;
            self.gravity = 2;
        }

        if self.time_gravity == 0 && self.gravity < self.max_gravity && !self.grounded {
            self.gravity += 1;
            self.time_gravity = 5;
        }
        if self.time_gravity > 0 {
            self.time_gravity -= 1;
        }

        //Update Sprite
        self.update_sprite(true);
    }

    pub fn render<G>(&mut self, c: piston_window::Context, g: &mut G) 
        where G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>
    {
        self.get_current_sprite().draw(c.transform, g);
    }
}