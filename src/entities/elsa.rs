use sprite::*;
use piston_window::*;

use crate::entities::Platform;
use crate::utils::{sprite::load_sprite, Position, CollisionInfo};

#[derive(Copy, Clone, PartialEq)]
pub enum ElsaAnimations {
    IDLE,
    WALK
}

pub struct Elsa {
    pub pos: Position,
    pub idle: Vec<Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>>,
    pub walk: Vec<Sprite<gfx_texture::Texture<gfx_device_gl::Resources>>>,
    pub sprite: usize,
    max_gravity: i8,
    pub gravity: i8,
    time_gravity: u8,
    pub grounded: bool,
    pub movements: [bool;3],
    pub facing_left: bool,
    jumping: bool,
    pub anim: ElsaAnimations,
    anim_time: u8
}

impl Elsa {
    pub fn new(factory: &mut gfx_device_gl::Factory) -> Elsa {
        let mut idle = vec![
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (1).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (2).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (3).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (4).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (5).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (6).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (7).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (8).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (9).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (10).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (11).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (12).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (13).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (14).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (15).png"),
            load_sprite(factory, "./resources/images/Elsa/Idle/Idle (16).png")
        ];

        let mut walk = vec![
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (1).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (2).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (3).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (4).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (5).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (6).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (7).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (8).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (9).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (10).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (11).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (12).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (13).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (14).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (15).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (16).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (17).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (18).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (19).png"),
            load_sprite(factory, "./resources/images/Elsa/Walk/Walk (20).png")
        ];

        for i in 0..idle.len() {
            idle[i].set_scale(0.25, 0.25);
        }
        for i in 0..walk.len() {
            walk[i].set_scale(0.25, 0.25);
        }

        Elsa {
            pos: Position::new(100, 100),
            idle: idle,
            walk: walk,
            sprite: 0,
            max_gravity: 5,
            gravity: 5,
            time_gravity: 5,
            grounded: false,
            movements: [false, false, false],
            facing_left: false,
            jumping: false,
            anim: ElsaAnimations::IDLE,
            anim_time: 3
        }
    }

    pub fn get_current_sprite(&mut self) -> &mut Sprite<gfx_texture::Texture<gfx_device_gl::Resources>> {
        match self.anim {
            ElsaAnimations::IDLE => return &mut self.idle[self.sprite],
            ElsaAnimations::WALK => return &mut self.walk[self.sprite]
        }
    }

    pub fn update_sprite(&mut self) {
        let x = self.pos.x;
        let y = self.pos.y;
        self.get_current_sprite().set_position(x as f64, y as f64);

        let face = self.facing_left;
        self.get_current_sprite().set_flip_x(face);
    }

    pub fn can_go(&mut self, position: &Position, platforms: &Vec<Platform>, win: &Platform) -> CollisionInfo {
        let elsa_bounding = self.get_current_sprite().bounding_box();
        let px = position.x as f64 - elsa_bounding[2] / 2.;
        let py = position.y as f64 - elsa_bounding[3] / 2.;

        for i in 0..platforms.len() {
            let pls = platforms[i].sprite.bounding_box();
            let plx = pls[0];
            let ply = pls[1];
            if px < plx + pls[2] && px + elsa_bounding[2] > plx && py < ply + pls[3] && py + elsa_bounding[3] > ply {
                return CollisionInfo::new(false, "Platform");
            }
        }

        let ws = win.sprite.bounding_box();
        let wx = ws[0];
        let wy = ws[1];
        if px < wx + ws[2] && px + elsa_bounding[2] > wx && py < wy + ws[3] && py + elsa_bounding[3] > wy {
            return CollisionInfo::new(false, "Win");
        }

        CollisionInfo::new(true, "")
    }

    pub fn update(&mut self, platforms: &Vec<Platform>, win: &Platform) -> bool {
        //Mouvements
        let mut pos = self.pos;

        if self.movements[0] { 
            pos.x -= 5; 
            if !self.facing_left {
                self.facing_left = true;
            }
        }
        if self.movements[1] { 
            pos.x += 5; 
            if self.facing_left {
                self.facing_left = false;
            }
        }

        let coll = self.can_go(&pos, platforms, win);
        if coll.can_go {
            self.pos = pos;
        }

        if coll.from == "Win" {
            return true;
        }

        //Jump
        if self.movements[2] {
            if self.grounded && ! self.jumping {
                self.grounded = false;
                self.jumping = true;
                self.gravity = -self.max_gravity;
            }
        }
        else {
            self.jumping = false;
        }

        //Update Gravity
        let futurpos = Position::new(self.pos.x, self.pos.y + self.gravity as i32);
        let coll = self.can_go(&futurpos, platforms, win);
        if coll.can_go {
            self.grounded = false;
            self.pos = futurpos;
        }
        else if self.gravity > 0 {
            self.grounded = true;
            self.gravity = 2;
        }

        if coll.from == "Win" {
            return true;
        }

        if self.time_gravity == 0 && self.gravity < self.max_gravity && !self.grounded {
            self.gravity += 1;
            self.time_gravity = 5;
        }
        if self.time_gravity > 0 {
            self.time_gravity -= 1;
        }

        //Animation
        if self.movements[0] || self.movements[1] {
            if self.anim == ElsaAnimations::IDLE {
                self.anim = ElsaAnimations::WALK;
                self.sprite = 0;
            }
        }
        else if self.anim == ElsaAnimations::WALK {
            self.anim = ElsaAnimations::IDLE;
            self.sprite = 0;
        }

        if self.anim_time == 0 {
            self.sprite += 1;

            let len;
            match self.anim {
                ElsaAnimations::IDLE => len = self.idle.len(),
                ElsaAnimations::WALK => len = self.walk.len()
            }
            if self.sprite == len  {
                self.sprite = 0;
            }

            self.anim_time = 3;
        }

        self.anim_time -= 1;

        self.update_sprite();
        false
    }

    pub fn input(&mut self, key: Key, is_press: bool) {
        match key {
            Key::Left | Key::Q => self.movements[0] = is_press,
            Key::Right | Key::D => self.movements[1] = is_press,
            Key::Up | Key::Z => self.movements[2] = is_press,
            _ => ()
        }
    }

    pub fn render<G>(&mut self, c: piston_window::Context, g: &mut G) 
        where G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>
    {
        self.get_current_sprite().draw(c.transform, g);
    }
}