use piston_window::*;
use crate::utils::{Position, TextRender, Config};
use crate::{States, AppInfo};

#[derive(Clone)]
struct OptionGameBool {
    name: TextRender,
    value_text: TextRender,
    value: usize,
    values: Vec<bool>
}

impl OptionGameBool {
    pub fn new(name: &str, value: usize, values: Vec<bool>, pos: [Position;2]) -> OptionGameBool {
        OptionGameBool {
            name: TextRender::new(text::Text::new_color(color::WHITE, 25), name, pos[0]),
            value_text: TextRender::new(text::Text::new_color(color::WHITE, 25), &*values[value].to_string(), pos[1]),
            value: value,
            values: values
        }
    }

    pub fn change_value(&mut self, left: bool) -> bool {
        if left && self.value > 0 {
            self.value -= 1;
            self.value_text.text = self.values[self.value].to_string();
        }
        else if !left && self.value < 1 {
            self.value += 1;
            self.value_text.text = self.values[self.value].to_string();
        }
        self.values[self.value]
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        self.name.draw(c, g, device, glyphs);
        self.value_text.draw(c, g, device, glyphs);
    }
}

#[derive(Clone)]
pub struct Options {
    texts: Vec<TextRender>,
    fps: OptionGameBool,
    unlimited_icebox: OptionGameBool,
    selected_option: usize,
    config: Config,
    win_size: Size
}

impl Options {
    pub fn new(win_size: Size, config: Config) -> Options {
        let fps;
        match config.get("fps").as_bool().expect("[Config] FPS value must be boolean") {
            true => fps = 0,
            false => fps = 1
        }
        let unlimited_icebox;
        match config.get("unlimited_icebox").as_bool().expect("[Config] Unlimited Icebox value must be boolean") {
            true => unlimited_icebox = 0,
            false => unlimited_icebox = 1
        }

        let fps_pos = [Position::new(win_size.width as i32 / 2 - 190, win_size.height as i32 / 2 - 51), Position::new(win_size.width as i32 / 2 + 80, win_size.height as i32 / 2 - 51)];
        let unlimited_pos = [Position::new(win_size.width as i32 / 2 - 200, win_size.height as i32 / 2), Position::new(win_size.width as i32 / 2 + 80, win_size.height as i32 / 2)];

        Options {
            texts: vec![TextRender::new(text::Text::new_color(color::WHITE, 35), "Options", Position::new(win_size.width as i32 / 2 - 80, win_size.height as i32 / 2 - 150))],
            fps: OptionGameBool::new("Show FPS", fps, vec![true, false], fps_pos),
            unlimited_icebox: OptionGameBool::new("IceBox Infini", unlimited_icebox, vec![true, false], unlimited_pos),
            selected_option: 0,
            config: config,
            win_size: win_size
        }
    }

    pub fn draw<G : Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(&mut self, c: piston_window::Context, g: &mut G, device: &mut gfx_device_gl::Device, glyphs: &mut Glyphs) {
        for i in 0..self.texts.len() {
            self.texts[i].draw(c, g, device, glyphs);
        }
        self.fps.draw(c, g, device, glyphs);
        self.unlimited_icebox.draw(c, g, device, glyphs);
        let text_pos: Position;
        match self.selected_option {
            0 => text_pos = self.fps.value_text.position,
            1 => text_pos = self.unlimited_icebox.value_text.position,
            _ => return
        }
        let polygon1 = [ [ (text_pos.x - 30) as f64, (text_pos.y - 10) as f64 ], [ (text_pos.x - 30) as f64, text_pos.y as f64 ], [ (text_pos.x - 40) as f64, (text_pos.y - 5) as f64] ];
        let polygon2 = [ [ (text_pos.x + 100) as f64, (text_pos.y - 10) as f64 ], [ (text_pos.x + 100) as f64, text_pos.y as f64 ], [ (text_pos.x + 110) as f64, (text_pos.y - 5) as f64] ];
        polygon(color::WHITE, &polygon1, c.transform, g);
        polygon(color::WHITE, &polygon2, c.transform, g);
    }

    pub fn input(&mut self, button: &Button, is_press: bool, info: AppInfo) -> AppInfo {
        let mut info = info;
        if is_press {
            if let Button::Keyboard(key) = *button {
                match self.selected_option {
                    0 => {
                        let mut value = info.fps;
                        match key {
                            Key::Left => value = self.debug.change_value(true),
                            Key::Right => value = self.debug.change_value(false),
                            _ => ()
                        }
                        self.config.set_bool("fps", value);
                        info.fps = value;
                    },
                    1 => {
                        let mut value = info.unlimited_icebox;
                        match key {
                            Key::Left => value = self.unlimited_icebox.change_value(true),
                            Key::Right => value = self.unlimited_icebox.change_value(false),
                            _ => ()
                        }
                        self.config.set_bool("unlimited_icebox", value);
                        info.unlimited_icebox = value;
                    },
                    _ => ()
                }
                match key {
                    Key::Up => if self.selected_option > 0 { self.selected_option -= 1 },
                    Key::Down => if self.selected_option < 1 { self.selected_option += 1 },
                    Key::Return => { self.config.save(); info.state = States::Menu;},
                    _ => ()
                }
            }
        }
        info
    }

    pub fn update(&mut self, info: AppInfo) -> AppInfo { info }
}