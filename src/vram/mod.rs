pub mod framebuffer;

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
