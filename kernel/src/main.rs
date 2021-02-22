#![no_std]
#![no_main]

#![feature(const_mut_refs)]
#![feature(fmt_as_str)]
#![feature(panic_info_message)]
#![feature(assoc_char_funcs)]

#![feature(abi_x86_interrupt)]

#[macro_use] extern crate log;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use x86_64::{
    VirtAddr,
    structures::paging::Page,
};

#[macro_use] pub mod framebuffer;
use framebuffer::FRAMEBUFFER;
use framebuffer::logger;

pub mod interrupts;
pub mod gdt;
pub mod memory;

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
        panic!("Framebuffer not found!");
    };

    FRAMEBUFFER.lock().clear(80, 13, 144);
    logger::init().expect("Failed to initialize logger!");
    debug!("Hello world!");

    gdt::init();
    interrupts::init_idt();

    println!("Mem offset boot_info: {:?}", boot_info.physical_memory_offset);
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().expect("Failed to locate physical memory offset!"));
    println!("Mem offset selected: {:?}", phys_mem_offset);

    let mut memory_mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf));
    unsafe {
        memory::map_page(page, &mut memory_mapper, &mut frame_allocator).expect("Failed to map page!");
    }
    trace!("Page mapped!");

    // x86_64::instructions::interrupts::int3();
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

    println!("It didn't crash!");

    loop {}
}
