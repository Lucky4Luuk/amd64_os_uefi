#![no_std]
#![no_main]

#![feature(const_mut_refs)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

pub mod framebuffer;
use framebuffer::FRAMEBUFFER;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    *FRAMEBUFFER.lock() = if let Some(mut_fb) = boot_info.framebuffer.as_mut() {
        framebuffer::Framebuffer::with_buf(mut_fb)
    } else {
        panic!("Framebuffer not found!"); //TODO: This really cannot panic, because there's no way to output to the screen
    };

    // FRAMEBUFFER.lock().set_pixel(0,0);
    FRAMEBUFFER.lock().clear();

    loop {}
}
