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
pub extern "C" fn _start() -> ! {

    osirs::init();
    /*osirs::halt();*/
    
    #[cfg(test)]
    test_main();

    loop{}
}

