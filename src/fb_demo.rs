#![no_std]
#![no_main]
#![feature(asm)]

use core::ptr::{read_volatile, write_volatile};

extern crate nds_panic;
extern crate nds_rt;

use core::fmt::Write;

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

#[no_mangle]
pub fn _irq_handler() {}

#[no_mangle]
pub fn main() {
    assert!(nds_rt::bios::div(203, 10) == (20, 3, 20));
    assert!(nds_rt::bios::sqrt(100) == 10);
    assert!(false);
    unsafe {
        write_volatile(POWER_CR, 0x3); // turn on bottom screen
        write_volatile(DISP_CNT, 0x00020000); // draw framebuffer to bottom screen
        write_volatile(REG_IME, 1); //enable interrupts
    }

    let bank_a = unsafe { vram::BANK_A };
    let mut bank_a = bank_a.framebuffer();

    let mut sat = 0;
    let mut rising = false;

    let mut frame = 0;

    let res = 32;

    loop {
        for y in 0..res {
            for x in 0..res {
                let c = ((y + frame) ^ (x + frame)) as u8 * (256 / res) as u8;
                let color =
                    color::HighColor::new(c, c.saturating_sub(255 - sat), c.saturating_sub(sat));
                let x = x * (256 / res);
                let y = y * (192 / res);
                for x2 in 0..(256 / res) {
                    for y2 in 0..(192 / res) {
                        bank_a.draw(x + x2, y + y2, color);
                    }
                }
            }
        }

        bank_a.draw_str("Hello world", 0, 0);

        if sat == 0 || sat == 255 {
            rising = !rising;
        }
        if rising {
            sat += 1;
        } else {
            sat -= 1;
        }
        frame += 1;
        nds_rt::bios::wait_vblank();
    }
}
