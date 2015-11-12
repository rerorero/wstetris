use std::collections::BTreeMap;

pub enum ServerCommand {
	Join {is_player: bool, player_id: u8},
	Fin,
	Board {state: Vec<Vec<u8>>, score: u8, player: u8, spectator: u8},
	GameOver
}

impl ServerCommand {
	pub fn to_blob(cmd: &ServerCommand) -> Vec<u8> {
		let mut blob: Vec<u8> = Vec::new();
		match *cmd {
			ServerCommand::Join {is_player: ref is_player, player_id: id} => {
				blob.push(0);  // op
				if *is_player {
					blob.push(1);
				} else {
					blob.push(0);
				}
				blob.push(id);
			},

			ServerCommand::Fin => {
				blob.push(1);  // op
			},

			ServerCommand::Board {state: ref state, score: score, player: player, spectator: spectator} => {
				blob.push(2);  // op
				blob.push(score);  // score
				blob.push(player);  // player
				blob.push(spectator);  // 観戦
				blob.push(state.len() as u8); // col size
				blob.push(state[0].len() as u8); // row size
				let mut clone = state.clone();
				for row in clone {
					blob.extend(row.into_iter());
				}
			},

			ServerCommand::GameOver => {
				blob.push(3);  // op
			}
		}
		return blob;
	}
}
