pub const SIZE: usize = 0x80;
static mut BUS: [u8; SIZE] = [0u8; SIZE];
static mut USED: [u8; SIZE] = [0u8;SIZE];

pub fn malloc(s:usize) -> Option <usize> {
    unsafe {
        //Intialize BUS
        //Reserve a block of s bytes
        for i in 0..(SIZE - s + 1) {
            let mut found_space = true;
            for j in 0..s {
                if USED[i + j] == 1 {
                    found_space = false;
                    break;
                }
            }

        
        //Scan for a contiguous region of size s
        //In s > 8, word level allocation
        //"Could be more efficient" it's an excersize!
        
            if found_space {
                for j in 0..s {
                    USED[i + j] = 1;
                }
                return Some(i);
            }
        }
    }
    return None;
}

pub fn free(address: usize, size: usize){
    unsafe{
        for i in 0..size{
            if address + i <SIZE {
                USED[address + i] = 0;
            }
        }
    }
}

pub fn setter(value: i32, address: usize) {
    unsafe{
        let ptr = (&mut BUS[address] as *mut u8) as *mut i32;
        *ptr = value;
    }
}

pub fn getter(address: usize) -> i32 {
    unsafe {
        let ptr = (&BUS[address] as *const u8) as *const i32;
        *ptr
    }
}


pub fn init(){
    unsafe {
        assert!((SIZE & (SIZE - 1)) == 0);
    }
}
