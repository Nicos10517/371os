// main.rs


#![no_main] //Tells Rust not to use the standard library
#![no_std] //Tells Rust not using standard "startup code"
           
//This problem is essentially moving us from Normal rust to OS rust

#[unsafe(no_mangle)] //Forbidding Rust from changing name of function

pub extern "C" fn _start() -> ! {

    loop{} //Infinite loop
}

use core::panic::PanicInfo; 

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//The linker is to map code to a physical memory access
//Rust defaults to building an executable for current computer, so if we want to build for a
//different computer, we need to tell Rust which one


//Fails if we try to run without library
//Need to tell compiler to abort when it fails*/


