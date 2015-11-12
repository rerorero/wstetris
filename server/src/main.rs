extern crate websocket;
extern crate rand;
extern crate rustc_serialize;

mod board;
mod block_state;
mod mino;
mod user;
mod user_rx;
mod server_command;
mod app;
mod game;

use std::thread;
use std::sync::*;
use std::sync::mpsc::RecvError;
use websocket::{Server, Message, Receiver, Sender};
use user::*;
use user_rx::*;
use app::*;

fn main() {
	let server = Server::bind("127.0.0.1:8001").unwrap();
    println!("WS server started.");

	let app = Arc::new(Mutex::new(App::new()));
	{ app.lock().unwrap().start(); }

	for connection in server {
	    println!("connection in server");

		let (ws_tx, ws_rx) = mpsc::channel();
		let app_arc = app.clone();

		thread::spawn(move || {
			let request = connection.unwrap().read_request().unwrap(); // Get the request
			request.validate().unwrap(); // Validate the request
			let response = request.accept(); // Form a response
			let client = response.send().unwrap(); // Send the response
			let session: Arc<Mutex<UserSession>>;
			{ session = app_arc.lock().unwrap().add_new_session(ws_tx.clone()); }

			let (sender, mut receiver) = client.split();
			let arc_sender = Arc::new(Mutex::new(sender));

			thread::spawn(move || {
				let local_sender = arc_sender.clone();
				loop {
					let cmd: Result<Vec<u8>, RecvError> = ws_rx.recv();
					match cmd {
						Ok(blob) => {
							let mut s = local_sender.lock().unwrap();
							s.send_message(Message::Binary(blob)).unwrap();
						}
						Err(e) => {
							println!("ws_rx recv() error!!, {:?}", e);
							return;
						}
					}
				}
			});

			for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					Message::Close(_) => {
						session.lock().unwrap().on_user_rx(UserRx::Closed);
						return;
					},
					Message::Text(s) => {
						let cmd = UserRx::fromJson(s);
						session.lock().unwrap().on_user_rx(cmd);
					},
					d => {
						println!("unknown message {:?}", d);
					},
				}
			}
			println!("finish incoming message");
		});
	}
}
