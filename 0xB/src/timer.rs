use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use crate::println;

static mut _TIMER: Timer = Timer { hrs: 0, min: 0, sec: 0};

#[allow(static_mut_refs)]
pub fn get_timer() -> & 'static mut Timer {
    unsafe {
        &mut _TIMER
    }
}

#[derive(Debug)]
pub struct Timer {
    hrs: u8,
    min: u8,
    sec: u8,
}

pub static mut CHARS: [u8; 6] = [0; 6];
pub static mut INDEX: usize = 0;


pub fn init_timer() {
    unsafe {
        let hrs: u8 = str::from_utf8_unchecked(&CHARS[0..2]).parse().unwrap();
        //println!("hrs {}", hrs);
        let min: u8 = str::from_utf8_unchecked(&CHARS[2..4]).parse().unwrap();
        //println!("min {}", min);
        let sec: u8 = str::from_utf8_unchecked(&CHARS[4..6]).parse().unwrap();
        //println!("sec {}", sec);

        *get_timer() = Timer::new_unchecked(hrs, min, sec);
    }
}


//Take as u8 from keyboard
impl Timer {
    pub unsafe fn new_unchecked(hrs: u8, min:u8, sec: u8) -> Timer {
        Timer {hrs, min, sec}
    }

    pub fn new(hrs: u8, min: u8, sec: u8) -> Option<Timer> {
        Some(Timer {hrs, min, sec})
    }

    pub fn tick(&mut self) {
        let Timer {hrs, min, sec} = *self;

        let inc_sec = sec + 1;
        let new_sec = (inc_sec) % 60;
        
        let inc_min = min + (inc_sec / 60);
        let new_min = inc_min % 60;

        let new_hrs = (hrs + (inc_min / 60)) % 24;

        *self = Timer {hrs: new_hrs, min: new_min , sec: new_sec };
    }

}

impl Display for Timer {
    fn fmt (&self, f:&mut Formatter <'_>) -> fmt::Result {
        
        let Timer {hrs, min, sec } = *self;
        write!(f, "{hrs:02}:{min:02}:{sec:02}")
    }

}
