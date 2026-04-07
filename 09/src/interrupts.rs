use lazy_static::lazy_static;



lazy_static! {
    static ref IDT: x86_64::structures::idt::InterruptDescriptorTable = {
        let mut idt = x86_64::structures::idt::InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler (stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    crate::println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
