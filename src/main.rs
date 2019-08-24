#![no_std]
#![no_main]

use core::ptr::write_volatile;

mod color;
mod vram;

const POWER_CR: *mut u32 = 0x04000304 as *mut u32;
const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;
const VRAMCNT_A: *mut u32 = 0x04000240 as *mut u32;
const REG_IE: *mut u32 = 0x04000210 as *mut u32;
const REF_IF: *mut u32 = 0x04000214 as *mut u32;

const RED: u16 = 31;
const VRAM: *mut [u16; 96_000] = 0x06800000 as *mut [u16; 96_000];
const VRAM_B: *mut [u16; 96_000] = 0x06820000 as *mut [u16; 96_000];

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(POWER_CR, 0x3);
    }

    let vram = vram::bank_a();
    let mut fb = vram.into_fb();

    let mut color = color::HighColor::new(0, 0, 0);

    for y in 0..128 {
        for x in 0..128 {
            color.red = (y ^ x) as u8 * 2;
            color.blue = (y ^ x) as u8 * 2;
            color.green = (y ^ x) as u8 * 2;

            fb.draw(x + 64, y + 32, color.into());
        }
    }

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
