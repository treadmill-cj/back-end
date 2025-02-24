use std::sync::{mpsc::Receiver, Arc, Mutex};

use websocket::{sync::Server, OwnedMessage};

use crate::CalcData;

pub fn run(rx: Receiver<CalcData>, connected: Arc<Mutex<bool>>) {
  let server = Server::bind("127.0.0.1:5000").unwrap();

	'connection: for request in server.filter_map(Result::ok) {
    println!("1 ws connection");

    let mut client = request.use_protocol("rust-websocket").accept().unwrap();
    println!("2 accepted");

    *connected.lock().unwrap() = true;

    loop {
      let data = rx.recv().unwrap();
      let str = format!("{{\"data\":{{\"total_distance\": {}, \"total_time_ms\": {}, \"speed\": {}}}}}",
        data.total_distance,
        data.total_time_ms,
        data.speed
      );
      println!("3 Sending data: {str}");
      let message = OwnedMessage::Text(str);
      match client.send_message(&message) {
        Ok(_) => println!("4 ok"),
        Err(e) => {
          println!("{e:?}");
          *connected.lock().unwrap() = false;
          continue 'connection
        },
      }
    }
	}
}