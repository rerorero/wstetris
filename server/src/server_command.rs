use std::collections::BTreeMap;

pub enum ServerCommand {
	Join {is_player: bool},
	Fin,
	Board {state: Vec<Vec<u8>>},
	GameOver
}

impl ServerCommand {
	pub fn to_blob(cmd: &ServerCommand) -> Vec<u8> {
		let mut blob: Vec<u8> = Vec::new();
		match *cmd {
			ServerCommand::Join {is_player: ref is_player} => {
				blob.push(0);  // op
				if *is_player {
					blob.push(1);
				} else {
					blob.push(0);
				}
			},

			ServerCommand::Fin => {
				blob.push(1);  // op
			},

			ServerCommand::Board {state: ref state} => {
				blob.push(2);  // op
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
