const VRAM_A: *mut u16 = 0x06800000 as *mut u16;
use core::ptr::write_volatile;

pub struct Uninitialized;
pub struct FrameBuffer;

const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;
const VRAMCNT_A: *mut u32 = 0x04000240 as *mut u32;

pub struct VBankA<M>{
    mode: M
}

pub fn bank_a() -> VBankA<Uninitialized>{
    VBankA{mode: Uninitialized}
}

impl VBankA<Uninitialized>{
    pub fn into_fb(self) -> VBankA<FrameBuffer>{
        unsafe {
            write_volatile(VRAMCNT_A, 0x80);
            write_volatile(DISP_CNT, 0x2_0000);
        }
        VBankA {
            mode: FrameBuffer
        }
    }
}

impl VBankA<FrameBuffer>{
    pub fn draw(&mut self, x: usize, y: usize, color: u16){
        let x = x % 256;
        let y = y % 192;
        let offset = (y * 256) + x;
        self.raw_draw(offset, color);
    }

    pub fn raw_draw(&mut self, offset: usize, color: u16){
        unsafe {
            write_volatile(VRAM_A.offset(offset as isize), color);
        }
    }
}
