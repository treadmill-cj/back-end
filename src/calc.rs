use std::{sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, time::{Duration, Instant}};

use crate::{CalcData, BELT_LEN};



pub fn run(rx: Receiver<()>, tx: Sender<CalcData>, calculated_data: Arc<Mutex<Vec<CalcData>>>) {
  let mut total_distance: f64 = 0.0;

  let now = Instant::now();
  let mut last_dur = Duration::from_secs(0);

  loop {
    // recive
    rx.recv().unwrap();

    // get time since last event
    let elapsed = now.elapsed();
    let duration = elapsed - last_dur;
    last_dur = elapsed;

    // calculations
    total_distance += BELT_LEN;
    let speed = BELT_LEN / duration.as_millis() as f64 * 1000.0;

    // forward data to api and websocket
    let data = CalcData { total_distance, total_time: elapsed, speed };
    tx.send(data).unwrap();
    calculated_data.lock().unwrap().push(data);
  }
}