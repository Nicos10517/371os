#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]


#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
