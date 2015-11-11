use std::ops::Add;
use user_rx::Key;

pub type StateColor = u8;

// #[derive(Debug, Copy, Clone)];
// pub type BlockState = u8;
pub const STATE_EMPTY: u8 = 0;
pub const STATE_COLOR_MIN: u8 = 10;
pub const STATE_PILED_COLOR_MIN: u8 = 20;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pos {
	pub x: i32,
	pub y: i32
}

impl Pos {
	pub fn new(x: i32, y: i32) -> Pos {
		Pos {x: x, y: y}
	}

	pub fn next(&self, key: Key) -> Pos {
		match key {
			Key::LEFT   => Pos { x: self.x - 1, y: self.y },
			Key::RIGHT  => Pos { x: self.x + 1, y: self.y },
			Key::DOWN   => Pos { x: self.x,     y: self.y + 1 },
			Key::ROTATION   => self.clone()
		}
	}
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        return Pos::new(self.x + other.x, self.y + other.y);
    }
}
