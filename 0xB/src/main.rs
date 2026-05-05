#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(static_mut_refs)]

mod vga;



#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info);
    /*osirs::halt();*/
}


#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { osirs::memory::init(offset) };
    let mut frame_allocator = unsafe { osirs::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
    osirs::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };
        
    /*osirs::halt();*/
    
    /*
    let ptr = 0xdeadbeef as *mut u8;
    unsafe { *ptr = 42; }
    println!("It did not crash!");
    */
    
    #[cfg(test)]
    test_main();

    loop{}
}

