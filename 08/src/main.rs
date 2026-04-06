#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]    

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
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("I'm main");
    
    #[cfg(test)]
    test_main();

    loop{}
}

