pub mod framebuffer;

use core::fmt::Write;
use framebuffer::FrameBuffer;

pub mod mode {
    #[derive(Copy, Clone)]
    pub struct FrameBuffer;
    #[derive(Copy, Clone)]
    pub struct Unknown;
}

#[derive(Copy, Clone)]
pub struct BankA<M> {
    mode: M,
}

impl<M> BankA<M> {
    pub fn framebuffer(self) -> BankA<mode::FrameBuffer> {
        let mut bank = BankA {
            mode: mode::FrameBuffer,
        };
        bank.init();
        bank
    }
}

impl FrameBuffer for BankA<mode::FrameBuffer> {
    const VRAM: usize = 0x6800000;
    const CTRL: usize = 0x04000240;
}

pub static mut BANK_A: BankA<mode::Unknown> = BankA {
    mode: mode::Unknown,
};

impl Write for BankA<mode::FrameBuffer> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            static mut pos: (usize, usize) = (0, 0);
            pos = self.draw_str(s, pos.0, pos.1);
        }
        Ok(())
    }
}
