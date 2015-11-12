use block_state::*;
use board::*;
use mino::*;
use user_rx::*;
use server_command::*;
use std::collections::HashMap;
use std::cell::RefCell;

const MAX_PLAYER: usize = 4;

pub struct Player {
	pub id: i32,
	pub mino: Option<Mino>,
	pub key: Option<Key>
}

impl Player {
	pub fn new(id: i32) -> Player {
		return Player {
			id: id,
			mino: None,
			key: None
		};
	}
}

pub struct Game {
	pub board: Board,
	players: Vec<Player>,
	tick: u32,
	fall_velocity: u32,
	score: u8
}

const FALL_VELOCITY_INIT: u32 = 40;
const UP_INT: u32 = 200;

impl Game {
	pub fn new() -> Game {
		Game {
			board: Board::new(),
			players: Vec::new(),
			tick: 0,
			fall_velocity: FALL_VELOCITY_INIT,
			score: 0
		}
	}

	fn tick(&mut self) {
		self.tick += 1;
	}

	pub fn new_game(&mut self) {
		self.tick = 0;
		self.score = 0;
		self.fall_velocity = FALL_VELOCITY_INIT;
		self.board.empty();
		self.players.clear();
	}

	pub fn get_board_state(&self, session_num: usize) -> ServerCommand {
		let mut minos = Vec::new();
		let mut players = 0;
		for player in &self.players {
			 match player.mino {
			 	Some(ref m) => minos.push(m),
			 	None => {}
			 }
			 players += 1;
		}

		let state = self.board.to_server_cmd(&minos);
		let spector = if session_num < players {
			players - session_num
		} else {
			0
		};
		return ServerCommand::Board {
			state: state, score: self.score, player: players as u8, spectator: spector as u8
		};
	}

	pub fn on_key_press(&mut self, user_id: i32, key: Key) {
		if self.on_playing() {
			self.players.iter()
				.position(|u| u.id == user_id)
				.map(|i| self.players[i].key = Some(key));
		}
	}

	pub fn attempt_to_add_player(&mut self, id: i32) -> bool {
		if self.players.len() < MAX_PLAYER {
			self.players.push(Player::new(id));
			return true;
		} else {
			return false
		}
	}

	pub fn remove_player(&mut self, id: i32) {
		self.players.iter()
			.position(|u| u.id == id)
			.map(|i| self.players.remove(i));
	}

	fn on_playing(&self) -> bool {
		return self.players.len() > 0;
	}


	//------
	// main
	//------
	pub fn on_frame(&mut self) -> (bool, bool) {
		self.tick();
		if !self.on_playing() {
			return (false, false);
		}

		if (self.tick % UP_INT) == 0 {
			// だんだん早く
			if self.fall_velocity > 5 {
				println!("spped up!!");
				self.fall_velocity -= 1;
			}
		}

		// clear line?
		let cleared = self.clear_lines();

		// will fall?
		let fallen = self.fall();

		// keys?
		let moved = self.key_move();

		// will appear?
		let (appeared, game_over) = self.appear();

		//println!("on_frame() c: {}, f:{}, m:{}, a:{}, game_over={}", cleared, fallen, moved, appeared, game_over);
		return (cleared || fallen || moved || appeared, game_over);
	}

	// return (has_appeared, collision_with_piled)
	fn appear(&mut self) -> (bool, bool) {
		let mut appeared = false;

		for player in &mut self.players {
			match player.mino {
			 	Some(_) => {},
			 	None => {
					let mut new_mino = Mino::choose(
						STATE_COLOR_MIN + player.id as u8,
						STATE_PILED_COLOR_MIN + player.id as u8);
					let init_pos = Board::initial_mino_pos(player.id, MAX_PLAYER);
					new_mino.set_pos(init_pos);
					if self.board.collision_with_piled(&new_mino.current_occupied()) {
						self.board.pile(&new_mino);
						return (true, true);
					}
					appeared = true;
					player.mino = Some(new_mino);
				}
			}
		}
		return (appeared, false);
	}

	fn fall(&mut self) -> bool {
		if (self.tick % self.fall_velocity) != 0 {
			return false;
		}
		let mut updated = false;
		for player in &mut self.players {
			let mut mino_piled = false;
			match player.mino {
			 	Some(ref mut m) => {
					let next_pos = m.get_pos().next(Key::DOWN);
					let next = m.next_pos_occupied(next_pos);
					if self.board.collision_with_piled(&next) {
						self.board.pile(&m);
						mino_piled = true;
					} else {
						m.set_pos(next_pos);
					}
					updated = true;
				},
			 	None => {}
			}
			if mino_piled {
				player.mino = None;
				player.key = None;
			}
		}
		return updated;
	}

	fn key_move(&mut self) -> bool {
		let mut updated = false;

		let mut others: HashMap<i32, Vec<Pos>> = HashMap::new();
		for p in &self.players {
			let pos = match p.mino {
				Some(ref m)  => m.current_occupied(),
				None => Vec::new()
			};
			others.insert(p.id, pos);
		}

		for player in &mut self.players {
			let mut mino_piled = false;
			match player.mino {
				Some(ref mut m) => {
					match player.key {
						Some(key) => {
							let mino = RefCell::new(m);
							let mut next;
							if key == Key::ROTATION {
								next = mino.borrow().next_rotate_occupied();
							} else {
								next = mino.borrow().next_pos_occupied(mino.borrow().get_pos().next(key));
							}

							if (key == Key::DOWN) && self.board.collision_with_piled(&next) {
								let mm = mino.borrow();
								self.board.pile(*mm);
								mino_piled = true;
								updated = true;

							} else {
								let mut collision = false;
								'other_loop: for (id, op) in &others {
									if *id != player.id {
										if Mino::collision(&next, op) {
											collision = true;
											break 'other_loop;
										}
									}
								}
								if collision ||
									self.board.collision_with_wall(&next) ||
									self.board.collision_with_piled(&next) {
									// do nothing

								} else {
									if key == Key::ROTATION {
										mino.borrow_mut().rotate();
									} else {
										let next_pos = mino.borrow().get_pos().next(key);
										mino.borrow_mut().set_pos(next_pos);
									}
									updated = true;
								}
							}
						},
						None => {}
					}
				},
				None => {}
			}
			if mino_piled { player.mino = None; }
			player.key = None;
		}

		return updated;
	}

	fn clear_lines(&mut self) -> bool {
		let cleared = self.board.clear_lines();
		if cleared > 0 {
			self.score += cleared;
			return true;
		}
		return false;
	}
}
