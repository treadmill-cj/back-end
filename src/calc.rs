use std::{sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, time::Duration};

use crate::CalcData;



pub fn run(rx: Receiver<Duration>, tx: Sender<CalcData>, calculated_data: Arc<Mutex<Vec<CalcData>>>) {
  loop {
    let x = rx.recv().unwrap();
  }
}