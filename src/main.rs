#![no_std]
#![no_main]

use core::ptr::write_volatile;

mod color;
mod vram;

use vram::framebuffer::FrameBuffer;

const POWER_CR: *mut u32 = 0x04000304 as *mut u32;
const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(POWER_CR, 0x3); // turn on bottom screen
        write_volatile(DISP_CNT, 0x00020000); // draw framebuffer to bottom screen
    }

    let bank_a = unsafe { vram::BANK_A };
    let mut bank_a = bank_a.framebuffer();

    let mut color = color::HighColor::new(0, 0, 0);

    for y in 0..128 {
        for x in 0..128 {
            color.red = (y ^ x) as u8 * 2;
            color.blue = (y ^ x) as u8 * 2;
            color.green = (y ^ x) as u8 * 2;

            bank_a.draw(x + 64, y + 32, color.into());
        }
    }

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
