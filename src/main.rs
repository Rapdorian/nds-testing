#![no_std]
#![no_main]

use core::ptr::write_volatile;

mod vram;

const POWER_CR: *mut u32 = 0x04000304 as *mut u32;
const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;
const VRAMCNT_A: *mut u32 = 0x04000240 as *mut u32;
const REG_IE: *mut u32 = 0x04000210 as *mut u32;
const REF_IF: *mut u32 = 0x04000214 as *mut u32;

const RED: u16 = 31;
const VRAM: *mut [u16;96_000] = 0x06800000 as *mut [u16;96_000];
const VRAM_B: *mut [u16;96_000] = 0x06820000 as *mut [u16;96_000];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(POWER_CR, 0x3);
    }

    let vram = vram::bank_a();
    let mut fb = vram.into_fb();
    
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    
    for y in 0..192{
        for x in 0..256 {
            r = x ^ y;
            g = r;
            b = r;

            let r = r & 0b11111;
            let g = (g & 0b11111) << 5;
            let b = (b & 0b11111) << 10;
            let color = (r | g | b) as u16;
            fb.draw(x, y, color);
        }
    }

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
