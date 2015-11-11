use rustc_serialize::json;
use rustc_serialize::json::Json;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Key {
	LEFT, RIGHT, ROTATION, DOWN
}

#[derive(Debug, Clone)]
pub enum UserRx {
	KeyPress {key: Key},
	Closed,
	ParsingError(String)
}

/////////////////////////////////////////////////////////////
// Data
// {
//    "op" : key
//    "d" : ???
// }
/////////////////////////////////////////////////////////////
impl UserRx {
	pub fn fromJson(json: String) -> UserRx {
		match Json::from_str(&json) {
		 	Ok(root) => {
				let op = root.as_object()
				  .and_then( |r| r.get("op"))
				  .and_then( |op| op.as_string());
				let d = root.as_object()
				  .and_then( |r| r.get("d"));

				match (op, d) {
					(Some("key"), Some(data)) => match data.as_string() {
						Some("l") => UserRx::KeyPress{ key: Key::LEFT },
						Some("r") => UserRx::KeyPress{ key: Key::RIGHT },
						Some("d") => UserRx::KeyPress{ key: Key::DOWN },
						Some("t") => UserRx::KeyPress{ key: Key::ROTATION },
						e => UserRx::ParsingError(format!("Unknown key {:?}", e)),
					},
					(Some(e), _) => UserRx::ParsingError(format!("Unknown key {:?}", e)),
					(_, _) => UserRx::ParsingError(json)
				  }
			},
		 	Err(err) => UserRx::ParsingError("not json!".to_string())
		}
	}
}
