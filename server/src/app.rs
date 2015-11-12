use std::thread;
use std::sync::*;
use std::collections::HashMap;
use std::cell::RefCell;
use block_state::*;
use server_command::*;
use user::*;
use user_rx::*;
use board::*;
use game::*;


pub struct SessionMgr {
    pub sessions: RefCell<Vec<Arc<Mutex<UserSession>>>>
}

impl SessionMgr {
	pub fn new() -> SessionMgr {
		return SessionMgr {
			sessions: RefCell::new(Vec::new())
		};
	}

	pub fn add_new_session(&self, sender: mpsc::Sender<Vec<u8>>) -> Arc<Mutex<UserSession>> {
		println!("newSession1");
		let mut sess = self.sessions.borrow_mut();
		let mut user_candidate: Option<User> = Some(User::new(0));
		for i in 0..MAX_USER {
			user_candidate = Some(User::new(i));
			for s in sess.iter() {
                let exists_user = s.lock().unwrap();
				if exists_user.user.id == i {
					user_candidate = None;
				}
			}
			if user_candidate.is_some() {
				break;
			}
		}
		// @todo check option
		let user = user_candidate.unwrap();
		let s = Arc::new(Mutex::new(UserSession::new(user, sender)));
		sess.push(s.clone());
		return s;
	}

    pub fn remove_session(&self, user: &User) {
		let mut sess = self.sessions.borrow_mut();
        sess.iter()
            .position(|s| s.lock().unwrap().user.id == user.id)
            .map(|i| sess.remove(i));
    }

	pub fn pop_user_rx(&mut self) -> HashMap<User, UserRx> {
		let mut sess = self.sessions.borrow_mut();
		let mut m = HashMap::new();
		for s in sess.iter() {
			let mut us = s.lock().unwrap();
			us.pop_user_rx().map(|rx| m.insert(us.user, rx));
		}
		return m;
	}

    pub fn send_to(&self, user: &User, cmd: &ServerCommand) {
        let sess = self.sessions.borrow();
        for s in sess.iter() {
			let mut us = s.lock().unwrap();
            if us.user.id == user.id {
                us.send_to(cmd);
                return;
            }
        }
    }

    pub fn broadcast(&self, cmd: &ServerCommand) {
        let sess = self.sessions.borrow();
        for s in sess.iter() {
			s.lock().unwrap().send_to(cmd);;
        }
    }

    pub fn debug_session(&self) {
        let sess = self.sessions.borrow();
        println!("=== sessions ===");
        for s in sess.iter() {
			let us = s.lock().unwrap();
            println!("id:{:?}, rx:{:?}", us.user.id, us.rx);
        }
        println!("================");
    }
}

pub struct App {
    pub session_mgr: Arc<Mutex<SessionMgr>>,
    game: Arc<Mutex<Game>>
}

impl App {

	pub fn new() -> App {
		return App {
			session_mgr: Arc::new(Mutex::new(SessionMgr::new())),
            game: Arc::new(Mutex::new(Game::new()))
		}
	}

	pub fn start(&self) {
		let mgr = self.session_mgr.clone();
        let game_arc = self.game.clone();
		thread::spawn(move || {
			loop {
				//println!("thread tick");

				//{ mgr.lock().unwrap().debug_session(); }

				let user_rx;
                let mut session_num;
				{
                    let mut _mgr = mgr.lock().unwrap();
                    user_rx = _mgr.pop_user_rx();
                    session_num = _mgr.sessions.borrow().len();
                }

                {
                    let mut game = game_arc.lock().unwrap();
    				for (user, rx) in &user_rx {
    					//println!("user {:?}", user);
    					//println!("action {:?}", rx);

                        match *rx {
                            UserRx::Closed => {
                                game.remove_player(user.id);
                                mgr.lock().unwrap().remove_session(user);
                            },
                            UserRx::KeyPress{ key: ref key } => {
                                game.on_key_press(user.id, *key);
                            }
                            _ => {}
                        }
    				}

                    // tick
                    let (updated, game_over) = game.on_frame();
                    if game_over {
                        let _mgr = mgr.lock().unwrap();
                        let board_cmd = game.get_board_state(session_num);
    			        _mgr.broadcast(&board_cmd);
    			        _mgr.broadcast(&ServerCommand::GameOver);
                        game.new_game();

                    } else if (updated) {
                        let board_cmd = game.get_board_state(session_num);
    			        mgr.lock().unwrap().broadcast(&board_cmd);
                    }
                }

		 		thread::sleep_ms(TICK_INTERVAL);
			}
		});
	}

	pub fn add_new_session(&self, sender: mpsc::Sender<Vec<u8>>) -> Arc<Mutex<UserSession>> {
        let user_session = self.session_mgr.lock().unwrap().add_new_session(sender);
        {
            let session = user_session.lock().unwrap();
            let id = session.user.id;
            let is_player = self.game.lock().unwrap().attempt_to_add_player(id);
            session.send_to(&ServerCommand::Join {is_player: is_player, player_id: id as u8});
        }
		return user_session;
    }
}

const MAX_USER: i32 = 100;
const TICK_INTERVAL: u32 = 50;
