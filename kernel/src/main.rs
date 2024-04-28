
#![no_std]
#![no_main]
#![feature(panic_info_message)]


#[macro_use]
mod console;


mod sbi;
mod logging;
mod lang_items;


mod sync;
mod syscall;
mod trap;
mod task;
mod loader;
mod timer;
mod config;

#[path = "boards/qemu.rs"]
mod board;

use core::arch::global_asm;
use log::*;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

// fn main() {
//     // println!("Hello, world!");
// }

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    print_kernel_info();
    trap::init();
    loader::load_apps();
    println!("[kernel] load apps finished");
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    loop{}
}

fn clear_bss() {
    extern  "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)}
    });
}

fn print_kernel_info () {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    println!("[kernel] This is Kinako!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}

