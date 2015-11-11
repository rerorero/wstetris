use block_state::*;
use server_command::*;
use mino::*;

const COLS: usize = 16;
const ROWS: usize = 20;

#[derive(Debug)]
pub struct Board {
	states : [[u8; ROWS]; COLS],
}

impl Board {
	pub fn new() -> Board {
		return Board{
			states : [[STATE_EMPTY; ROWS]; COLS]
		};
	}

	pub fn empty(&mut self) {
		self.states = [[STATE_EMPTY; ROWS]; COLS];
	}

	pub fn to_server_cmd(&self, minos: &Vec<&Mino>) -> ServerCommand {
		let mut vector = Vec::new();
		for i in 0..COLS {
			vector.push(self.states[i].to_vec());
		}
		for mino in minos {
			let positions = mino.current_occupied();
			for pos in positions {
				vector[pos.x as usize][pos.y as usize] = mino.color;
			}
		}
		return ServerCommand::Board{ state: vector };
	}

	pub fn initial_mino_pos(num: i32, max_player: usize) -> Pos {
		return Pos::new((num * (COLS / max_player) as i32), 0);
	}

	pub fn collision_with_wall(&self, positions: &Vec<Pos>) -> bool {
		for pos in positions {
			if (pos.x < 0 || COLS as i32 <= pos.x) {
				return true;
			}
		}
		return false;
	}

	pub fn collision_with_piled(&self, positions: &Vec<Pos>) -> bool {
		for pos in positions {
			if (
				pos.x < 0 ||
				COLS as i32 <= pos.x ||
				pos.y < 0 ||
				ROWS as i32 <= pos.y ||
				self.states[pos.x as usize][pos.y as usize] != STATE_EMPTY
			) {
				return true;
			}
		}
		return false;
	}

	pub fn pile(&mut self, mino: &Mino) {
		let positions = mino.current_occupied();
		for pos in positions {
			self.states[pos.x as usize][pos.y as usize] = mino.piled_color;
		}
	}

	pub fn clear_lines(&mut self) -> u8 {
		let mut cleared = 0;
		for y in 0..ROWS {
			let mut fill = true;
			'row_loop: for x in 0..COLS {
				if self.states[x][y] == STATE_EMPTY {
					fill = false;
					break 'row_loop;
				}
			}
			if fill {
				cleared += 1;
				for yacc in 0..y {
					let yy = y - yacc;
					for xx in 0..COLS {
						if yy == 0 {
							self.states[xx][yy] = STATE_EMPTY;
						} else {
							self.states[xx][yy] = self.states[xx][yy-1];
						}
					}
				}
			}
		}
		return cleared;
	}
}


#[test]
fn test_board_new() {
	let b = Board::new();
	assert!(match b.states[0][0] {
		STATE_EMPTY => true,
		_ => false
	});
}
