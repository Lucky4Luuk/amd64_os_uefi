#![no_std]
#![no_main]

#![feature(const_mut_refs)]
#![feature(fmt_as_str)]
#![feature(panic_info_message)]

#![feature(assoc_char_funcs)]

#[macro_use] extern crate log;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

pub mod framebuffer;
use framebuffer::FRAMEBUFFER;

#[macro_use] use framebuffer::logger;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //QEMU debug output
    let qemu_debug_available = unsafe { cpuio::inb(0xe9) } == 0xe9;
    if qemu_debug_available {
        for c in "[PANIC] ".chars() {
            unsafe {
                cpuio::outb(c as u8, 0xe9);
            }
        }
        // let mut line = info.location().unwrap().line();
        // for i in 0..4 {
        //     //num / (base ^ digit) % base
        //     let num = line / 10u32.pow(i) % 10;
        //     unsafe {
        //         cpuio::outb(num as u8, 0xe9);
        //     }
        // }
        for c in info.message().unwrap().as_str().unwrap_or("no panic message provided").chars() {
            unsafe {
                cpuio::outb(c as u8, 0xe9);
            }
        }
    }

    //On-screen output
    println!("Panic: {}", info);

    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    *FRAMEBUFFER.lock() = if let Some(mut_fb) = boot_info.framebuffer.as_mut() {
        framebuffer::Framebuffer::with_buf(mut_fb)
    } else {
        panic!("Framebuffer not found!"); //TODO: This really cannot panic, because there's no way to output to the screen
    };

    FRAMEBUFFER.lock().clear(80, 13, 144);

    logger::init();

    debug!("Hello world!");

    debug!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

    // for i in 0..100 {
    //     error!("FUCK");
    // }

    panic!("FUCK");

    // for x in 0..10 {
    //     for y in 0..10 {
    //         FRAMEBUFFER.lock().set_pixel(x,y, 255, 0, 0); //x,y, r, g , b
    //     }
    // }

    loop {}
}
