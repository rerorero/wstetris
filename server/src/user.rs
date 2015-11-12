
use std::sync::*;
use std::sync::mpsc::*;
use server_command::*;
use user_rx::*;


pub type UserId = i32;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct User {
	pub id: UserId
}

impl User {
	pub fn new(id: UserId) -> User {
		return User {id: id};
	}
}

pub struct UserSession {
	pub user: User,
	ch: mpsc::Sender<Vec<u8>>,
	pub rx: Option<UserRx>
}

impl UserSession {
	pub fn new(user: User, ch: mpsc::Sender<Vec<u8>>) -> UserSession {
		return UserSession {
			user: user,
			ch: ch,
			rx: None
		};
	}

	pub fn on_user_rx(&mut self, cmd: UserRx) {
		match self.rx {
			Some(_) => println!("already queued."),
			None => self.rx = Some(cmd)
		};
	}

	pub fn pop_user_rx(&mut self) -> Option<UserRx> {
		let org = self.rx.clone();
		self.rx = None;
		return org;
	}

	pub fn send_to(&self, cmd: &ServerCommand) -> bool {
        match self.ch.send(ServerCommand::to_blob(&cmd)) {
            Ok(v) => true,
            Err(e) => {
				println!("send_to ch failed!! {}", e);
				false
			}
        }
	}
}
