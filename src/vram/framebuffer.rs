use crate::color::HighColor;
use core::ptr::write_volatile;

pub trait FrameBuffer {
    const VRAM: usize;
    const CTRL: usize;

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
}
