use std::{str::FromStr, sync::{Arc, Mutex}};

use tiny_http::{Response, Server, Header};

use crate::CalcData;

pub fn run(calculated_data: Arc<Mutex<Vec<CalcData>>>) {
  let server = Server::http("127.0.0.1:5001").unwrap();

  for request in server.incoming_requests() {

      match request.url() {
        "/all" => {
          println!("api data");
          let calculated_data = calculated_data.lock().unwrap();
          let mut total_str = String::from("[");
          for data in calculated_data.iter() {
            total_str += &format!("{{\"total_distance\": {}, \"total_time_ms\": {}, \"speed\": {}}},",
              data.total_distance,
              data.total_time_ms,
              data.speed
            );
          }
          total_str.pop(); // remove last comma
          total_str += "]";
          let h = Header::from_str("Access-Control-Allow-Origin: *").unwrap();
          let h2 = Header::from_str("Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS").unwrap();
          let h3 = Header::from_str("Access-Control-Allow-Headers: Content-Type, Authorization").unwrap();
          request.respond(Response::from_string(total_str).with_header(h).with_header(h2).with_header(h3)).unwrap()
        },
        _ => request.respond(Response::from_string("404")).unwrap()
      }
  }
}