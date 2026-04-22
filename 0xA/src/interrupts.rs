#![allow(static_mut_refs)]

use lazy_static::lazy_static;
use pic8259::ChainedPics;



lazy_static! {
    static ref IDT: x86_64::structures::idt::InterruptDescriptorTable = {
        let mut idt = x86_64::structures::idt::InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_handler);
        idt
    };
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: ChainedPics = {
    unsafe { pic8259::ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }
};



pub fn init_idt() {
    IDT.load();
    unsafe {
        PICS.initialize();
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn breakpoint_handler (stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler ( stack_frame: x86_64::structures::idt::InterruptStackFrame, error_code: u64) -> ! {
    assert!(error_code == 0);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler (stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    crate::println!("INTERRUPT: TIMER \n{:#?}", stack_frame);
    unsafe { PICS.notify_end_of_interrupt(InterruptIndex::Timer as u8) };
}


#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

