use std::sync::mpsc::Receiver;

use websocket::{sync::Server, OwnedMessage};

use crate::CalcData;

pub fn run(rx: Receiver<CalcData>) {
  let server = Server::bind("127.0.0.1:5000").unwrap();

	for request in server.filter_map(Result::ok) {
		// Spawn a new thread for each connection.
    println!("{:?}", request.protocols());

    let mut client = request.use_protocol("rust-websocket").accept().unwrap();

    let ip = client.peer_addr().unwrap();

    println!("Connection from {}", ip);

    let message = OwnedMessage::Text("{\"data\":\"hello\"}".to_string());
    client.send_message(&message).unwrap();
    let message = OwnedMessage::Text("{\"data\":\"hello2\"}".to_string());
    client.send_message(&message).unwrap();
    let message = OwnedMessage::Text("{\"data\":\"hello3\"}".to_string());
    client.send_message(&message).unwrap();

    loop {
      let _data = rx.recv().unwrap();
      let message = OwnedMessage::Text("{\"data\":\"calc data\"}".to_string());
    client.send_message(&message).unwrap();
    }
	}
}