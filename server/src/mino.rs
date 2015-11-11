use block_state::*;

#[derive(Copy, Clone)]
pub enum Shape {
	I, L, J, Z, S, O, T
}

impl Shape {
	pub fn choose() -> Shape {
		use rand;
		let variants = [Shape::I, Shape::L, Shape::J, Shape::Z, Shape::S, Shape::O, Shape::T];
		let i = rand::random::<usize>() % variants.len();
		return variants[i];
	}
 }

pub struct Mino {
	shape: Shape,
	patterns: [[Pos; 4]; 4],
	rotation: usize,
	pub color: StateColor,
	pub piled_color: StateColor,
	pos: Pos
}

impl Mino {
	pub fn new(shape: Shape, color: StateColor, piled_color: StateColor) -> Mino {
		Mino {
			shape: shape,
			rotation: 0,
			color: color,
			piled_color: piled_color,
			pos: Pos::new(0, 0),
			patterns:  match shape {
				Shape::I =>
					[
					    [Pos::new(0,1), Pos::new(1,1), Pos::new(2,1), Pos::new(3,1)],
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(1,3)],
					    [Pos::new(0,0), Pos::new(1,0), Pos::new(2,0), Pos::new(3,0)],
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(1,3)]
					],
				Shape::L =>
					[
					    [Pos::new(0,0), Pos::new(1,0), Pos::new(1,1), Pos::new(1,2)],
					    [Pos::new(0,1), Pos::new(1,1), Pos::new(2,1), Pos::new(2,0)],
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(2,2)],
					    [Pos::new(0,1), Pos::new(0,2), Pos::new(1,1), Pos::new(2,1)]
					],
				Shape::J =>
					[
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(1,2), Pos::new(0,2)],
					    [Pos::new(0,0), Pos::new(0,1), Pos::new(1,1), Pos::new(2,1)],
					    [Pos::new(2,0), Pos::new(1,0), Pos::new(1,1), Pos::new(1,2)],
					    [Pos::new(0,1), Pos::new(1,1), Pos::new(2,1), Pos::new(2,2)]
					],
				Shape::Z =>
					[
					    [Pos::new(0,0), Pos::new(1,0), Pos::new(1,1), Pos::new(2,1)],
					    [Pos::new(2,0), Pos::new(2,1), Pos::new(1,1), Pos::new(1,2)],
					    [Pos::new(0,0), Pos::new(1,0), Pos::new(1,1), Pos::new(2,1)],
					    [Pos::new(2,0), Pos::new(2,1), Pos::new(1,1), Pos::new(1,2)]
					],
				Shape::S =>
					[
					    [Pos::new(0,1), Pos::new(1,1), Pos::new(1,0), Pos::new(2,0)],
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(2,1), Pos::new(2,2)],
					    [Pos::new(0,1), Pos::new(1,1), Pos::new(1,0), Pos::new(2,0)],
					    [Pos::new(1,0), Pos::new(1,1), Pos::new(2,1), Pos::new(2,2)]
					],
				Shape::O =>
					[
					    [Pos::new(0,0), Pos::new(0,1), Pos::new(1,0), Pos::new(1,1)],
					    [Pos::new(0,0), Pos::new(0,1), Pos::new(1,0), Pos::new(1,1)],
					    [Pos::new(0,0), Pos::new(0,1), Pos::new(1,0), Pos::new(1,1)],
					    [Pos::new(0,0), Pos::new(0,1), Pos::new(1,0), Pos::new(1,1)]
					],
				Shape::T =>
					[
					    [Pos::new(1,1), Pos::new(1,0), Pos::new(0,1), Pos::new(2,1)],
					    [Pos::new(1,1), Pos::new(1,0), Pos::new(1,2), Pos::new(2,1)],
					    [Pos::new(1,1), Pos::new(0,1), Pos::new(1,2), Pos::new(2,1)],
					    [Pos::new(1,1), Pos::new(0,1), Pos::new(1,2), Pos::new(1,0)]
					]
				},
		}
	}

	fn next(state: usize) -> usize {
		return (state +1) % 4;
	}

	fn current_pattern(&self) -> [Pos; 4] {
		return self.patterns[self.rotation];
	}

	fn next_pattern(&self) -> [Pos; 4] {
		return self.patterns[Mino::next(self.rotation)];
	}

	pub fn choose(color: StateColor, piled_color: StateColor) -> Mino {
		return Mino::new(Shape::choose(), color, piled_color);
	}

	pub fn rotate(&mut self) {
		self.rotation = Mino::next(self.rotation);
	}

	pub fn set_pos(&mut self, pos: Pos) {
		self.pos = pos;
	}

	pub fn get_pos(&self) -> Pos {
		return self.pos;
	}

	fn occupied(pos: Pos, pattern: [Pos; 4]) -> Vec<Pos> {
		let mut v = Vec::new();
		for i in 0..pattern.len() {
			v.push(pattern[i] + pos);
		}
		return v;
	}

	pub fn current_occupied(&self) -> Vec<Pos> {
		return Mino::occupied(self.get_pos(), self.current_pattern());
	}

	pub fn next_rotate_occupied(&self) -> Vec<Pos> {
		return Mino::occupied(self.get_pos(), self.next_pattern());
	}

	pub fn next_pos_occupied(&self, pos: Pos) -> Vec<Pos> {
		return Mino::occupied(pos, self.current_pattern());
	}

	pub fn collision_with_minos(position: Vec<Pos>, others: Vec<Mino>) -> bool {
		for other in others {
			let ref pos = other.get_pos();
			if position.contains(pos) {
				return true;
			}
		}
		return false;
	}

	pub fn collision(position: &Vec<Pos>, others: &Vec<Pos>) -> bool {
		for other in others {
			if position.contains(other) {
				return true;
			}
		}
		return false;
	}
}

#[test]
fn test_mino_next() {
	let mut m = Mino::choose(1, 1);
	let v1 = m.current_pattern();
	m.rotate();
	m.rotate();
	m.rotate();
	let v2 = m.next_pattern();
	assert_eq!(v1, v2);
}
