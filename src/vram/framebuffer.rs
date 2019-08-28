use crate::color::HighColor;
use crate::font::{Font, TestFont};
use core::ptr::write_volatile;

use font8x8::legacy::BASIC_LEGACY;

static FONT: [[u8; 8]; 128] = BASIC_LEGACY;

pub trait FrameBuffer {
    const VRAM: usize;
    const CTRL: usize;
    const WHITE: HighColor = HighColor {
        red: 0xFF,
        blue: 0xFF,
        green: 0xFF,
    };
    const BLACK: HighColor = HighColor {
        red: 0,
        blue: 0,
        green: 0,
    };
    const RED: HighColor = HighColor {
        red: 0xFF,
        blue: 0x00,
        green: 0x00,
    };

    const GREEN: HighColor = HighColor {
        red: 0x00,
        blue: 0x00,
        green: 0xFF,
    };

    const BLUE: HighColor = HighColor {
        red: 0x00,
        blue: 0xFF,
        green: 0x00,
    };

    fn init(&mut self) {
        unsafe {
            write_volatile(Self::CTRL as *mut u32, 0x80);
        }
    }

    fn draw(&mut self, x: usize, y: usize, color: HighColor) {
        let vram = Self::VRAM as *mut u16;
        if x < 256 && y < 192 {
            unsafe {
                write_volatile(vram.offset(((y * 256) + x) as isize), color.into());
            }
        }
    }

    fn draw_hr(&mut self, height: usize, color: HighColor) {
        for x in 0..256 {
            self.draw(x, height, color);
        }
    }

    fn draw_glyph(&mut self, c: char, row: usize, line: usize) {
        let g = FONT[c as usize];
        let mut col = 0;
        for x in &g {
            for bit in 0..8 {
                let x_off = (row*9);
                let y_off = (line*10) + col;
                match *x & 1 << bit {
                    0 => {
                        self.draw(x_off+bit, y_off, Self::BLUE);
                    }
                    _ => {
                        self.draw(x_off+ bit, y_off, Self::WHITE);
                    }
                }
            }
            col += 1;
        }
    }
}
