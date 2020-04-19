#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
            alpha: 255
        }
    }

    pub fn get_float(&self) -> [f32; 4] {
        [self.red as f32 / 255., self.green as f32 / 255., self.blue as f32 / 255., self.alpha as f32 / 255.]
    }
}