#![no_std]
#![no_main]
#![feature(panic_info_message, lang_items, core_intrinsics, asm)]

use core::fmt::Write;
use core::intrinsics;
use core::ptr::{read_volatile, write_volatile};

use numtoa::NumToA;

mod color;
mod font;
mod vram;

use vram::framebuffer::FrameBuffer;

const POWER_CR: *mut u32 = 0x04000304 as *mut u32;
const DISP_CNT: *mut u32 = 0x04000000 as *mut u32;
const REG_IE: *mut u32 = 0x04000210 as *mut u32;
const REG_IF: *mut u32 = 0x04000218 as *mut u32;
const REG_IME: *mut u32 = 0x04000208 as *mut u32;
const REG_VCOUNT: *mut u16 = 0x04000006 as *mut u16;

fn wait_vblank() {
    unsafe { while read_volatile(REG_VCOUNT) < 192 {} }
}

#[no_mangle]
#[link_section = ".text.startup"]
pub extern "C" fn _start() -> ! {
    unsafe {
        write_volatile(POWER_CR, 0x3); // turn on bottom screen
        write_volatile(DISP_CNT, 0x00020000); // draw framebuffer to bottom screen
        write_volatile(REG_IME, 1); //enable interrupts
    }

    let bank_a = unsafe { vram::BANK_A };
    let mut bank_a = bank_a.framebuffer();

    let mut sat = 0;
    let mut rising = false;

    loop {
        for y in 0..128 {
            for x in 0..128 {
                let c = (y ^ x) as u8 * 2;
                let color =
                    color::HighColor::new(c, c.saturating_sub(255 - sat), c.saturating_sub(sat));
                bank_a.draw(x + 64, y + 32, color.into());
            }
        }
        if sat == 0 || sat == 255 {
            rising = !rising;
        }
        if rising {
            sat += 1;
        } else {
            sat -= 1;
        }

        bank_a.draw_str("Hello world", 0, 0);

        wait_vblank();
    }
}

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(panic: &PanicInfo<'_>) -> ! {
    wait_vblank();
    unsafe {
        write_volatile(POWER_CR, 0x3);
        write_volatile(DISP_CNT, 0x20000);
    }
    let bank_a = unsafe { vram::BANK_A };
    let mut bank_a = bank_a.framebuffer();

    for y in 0..192 {
        for x in 0..256 {
            let c = ((y + (y / 3)) ^ x) / 2;
            let c = c as u8;
            let color = color::HighColor::new(c, c, c);
            bank_a.draw(x, y, color.into());
        }
    }
    writeln!(bank_a, "{}", panic);
    wait_vblank();
    loop {}
}
