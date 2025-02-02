use std::{sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, time::{Duration, Instant}};

use crate::{CalcData, BELT_LEN};



pub fn run(rx: Receiver<()>, tx: Sender<CalcData>, calculated_data: Arc<Mutex<Vec<CalcData>>>, now: Arc<Mutex<Instant>>) {
  let mut total_distance: f64 = 0.0;

  let mut last_dur = Duration::from_secs(0);

  loop {
    // recive
    rx.recv().unwrap();

    // get time since last event
    let duration;
    let elapsed = now.lock().unwrap().elapsed();
    if last_dur > elapsed { // reset happened
      last_dur = elapsed;
      duration = elapsed;
    } else {
      duration = elapsed - last_dur;
      last_dur = elapsed;
    }

    // stop two events in a row with infinite speed
    if elapsed.as_millis() < 100 {
      continue;
    }

    // calculations
    total_distance += BELT_LEN;
    let speed = BELT_LEN / duration.as_millis() as f64 * 1000.0;

    // forward data to api and websocket
    let data = CalcData { total_distance, total_time_ms: elapsed.as_millis(), speed };
    println!("data queued");
    tx.send(data).unwrap();
    calculated_data.lock().unwrap().push(data);
  }
}