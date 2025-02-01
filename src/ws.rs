use std::sync::mpsc::Receiver;

use websocket::{sync::Server, OwnedMessage};

use crate::CalcData;

pub fn run(rx: Receiver<CalcData>) {
  let server = Server::bind("127.0.0.1:5000").unwrap();

	for request in server.filter_map(Result::ok) {

    let mut client = request.use_protocol("rust-websocket").accept().unwrap();

    let ip = client.peer_addr().unwrap();

    println!("Connection from {}", ip);

    loop {
      let data = rx.recv().unwrap();
      let str = format!("{{\"data\":{{\"total_distance\": {}, \"total_time_ms\": {}, \"speed\": {}}}}}",
        data.total_distance,
        data.total_time_ms,
        data.speed
      );
      println!("Sending data: {str}");
      let message = OwnedMessage::Text(str);
      client.send_message(&message).unwrap();
    }
	}
}