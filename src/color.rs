#[derive(Copy, Clone, Default)]
pub struct HighColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl HighColor {
    pub fn new(r: u8, g: u8, b: u8) -> HighColor {
        HighColor {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn normalize(&mut self) -> HighColor {
        HighColor {
            red: (self.red / 8) & 0x1F,
            green: (self.green / 8) & 0x1F,
            blue: (self.blue / 8) & 0x1F,
        }
    }

    fn stitch(&self) -> u16 {
        self.red as u16 | ((self.green as u16) << 5) | ((self.blue as u16) << 10)
    }
}

impl Into<u16> for HighColor {
    fn into(mut self) -> u16 {
        self.normalize().stitch()
    }
}
