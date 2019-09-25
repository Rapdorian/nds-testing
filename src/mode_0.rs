#![no_std]
#![no_main]
#![feature(asm)]

use core::ptr::{copy_nonoverlapping, read_volatile, write_volatile};
use nds_rt::registers::arm9::*;

extern crate nds_panic;
extern crate nds_rt;

// assets
const WOOD_TILES: &[u8] = include_bytes!("../assets/wood/tiles.bin");
const WOOD_MAP: &[u8] = include_bytes!("../assets/wood/map.bin");
const WOOD_PAL: &[u8] = include_bytes!("../assets/wood/pal.bin");

#[no_mangle]
pub fn _irq_handler() {}

#[no_mangle]
pub unsafe fn main() {
    write_volatile(POWCNT1, POWER_ALL_2D);

    write_volatile(DISPCNT, MODE_0_2D | DISPLAY_BG0_ACTIVE);
    write_volatile(
        BG0CNT,
        BG_32_32 | BG_COLOR_256 | bg::map_base(0) | bg::tile_base(1),
    );
    write_volatile(VRAMCNT_A, VRAM_ENABLE | VRAM_A_MAIN_BG);

    let (_, tiles, _) = WOOD_TILES.align_to::<u16>();
    let (_, map, _) = WOOD_MAP.align_to::<u16>();
    let (_, pal, _) = WOOD_PAL.align_to::<u16>();

    // copy tiles
    for (off, tile) in tiles.iter().enumerate() {
        write_volatile(bg::tile_ram(1).offset(off as isize), *tile);
    }
    // copy map
    for (off, tile) in map.iter().enumerate() {
        write_volatile(bg::map_ram(0).offset(off as isize), *tile);
    }
    // copy pal
    for (off, col) in pal.iter().enumerate() {
        write_volatile(BG_PALETTE.offset(off as isize), *col);
    }

    // init top screen
    write_volatile(DISPCNT_B, MODE_0_2D | DISPLAY_BG0_ACTIVE);
    // lets use the same tile data as main?
    write_volatile(
        BG0CNT_SUB,
        BG_32_32 | BG_COLOR_256 | bg::map_base(0) | bg::tile_base(1),
    );
    write_volatile(VRAMCNT_C, VRAM_ENABLE | VRAM_C_SUB_BG);

    /*for (off, col) in pal.iter().enumerate() {
        write_volatile(BG_PALETTE_SUB.offset(off as isize), off as u16);
    }*/

    let mut frame = 0;

    loop {
        nds_rt::bios::wait_vblank();
        write_volatile(BG0HOFS, frame);
        write_volatile(BG0VOFS, frame * 2);
        write_volatile(BG0HOFS_SUB, 0_u16.wrapping_sub(frame));
        write_volatile(BG0VOFS_SUB, 0_u16.wrapping_sub(frame * 2));

        frame += 1;
    }
}
