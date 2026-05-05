use crate::print;
use alloc::collections::VecDeque;

pub const FOOD: u8 = 0xA2; 
const HEIGHT: u32 = 25;
const WIDTH: u32 = 80;
pub static mut FOOD_POS: (u32, u32) = (0, 0);


pub static mut SNAKE: Snake = Snake {
    body: VecDeque::new(),
    direction: Direction::Up,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Snake {
    body: VecDeque<(u32, u32)>,
    pub direction: Direction,
}


pub fn init_snake() {
    unsafe {
        SNAKE.body.reserve_exact(25 * 80);
        SNAKE.body.push_front((WIDTH/2, HEIGHT/2));
        update_app();

    }
}

static mut FOOD_TICK: u32 = 1;

fn update_app() {
    unsafe {
        FOOD_TICK = FOOD_TICK.wrapping_add(1);
        let x = (FOOD_TICK.wrapping_mul(113)) % WIDTH;
        let y = (FOOD_TICK.wrapping_mul(47)) % HEIGHT;
        FOOD_POS =(x, y);
        crate::vga::write_cell(x as usize, y as usize, FOOD, 0x0C);
    }
}

impl Snake {
    pub fn snake_tick() {
    unsafe {
        let dir = SNAKE.direction;
        let (x, y) = SNAKE.body[0];

        let new_head = match dir {
            Direction::Up    => (x, (y + HEIGHT - 1) % HEIGHT),
            Direction::Right => ((x + 1) % WIDTH, y),
            Direction::Down  => (x, (y + 1) % HEIGHT),
            Direction::Left  => ((x + WIDTH - 1) % WIDTH, y),
        };

        SNAKE.body.push_front(new_head);
        crate::vga::write_cell(new_head.0 as usize, new_head.1 as usize, 0xDB, 0x0A);

        if new_head == FOOD_POS {
            update_app();
        } else {
            let old_tail = SNAKE.body.pop_back().unwrap();
            crate::vga::write_cell(old_tail.0 as usize, old_tail.1 as usize, b' ', 0x00);
        }
    }
}

    pub fn snake_pos(&mut self, direction: Direction) -> ((u32, u32), (u32, u32)) {
        // let (mut head,   mut tail) = self.body[0];

        let head: (u32, u32) = self.body[0];

        let mut new_head: (u32, u32) = (WIDTH/2, HEIGHT/2);

        let (x, y) = head;

        new_head = match direction {
            Direction::Up => (x, (y + HEIGHT - 1) % HEIGHT),
            Direction::Right => ((x + 1) % WIDTH, y),
            Direction::Down => (x, (y + 1) % HEIGHT),
            Direction::Left => ((x + WIDTH - 1) % WIDTH, y),
        };

        self.body.push_front(new_head);
        let old_tail = self.body.pop_back().unwrap();

        (new_head, old_tail)
    }
}
  


