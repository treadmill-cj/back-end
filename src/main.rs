use std::{sync::{mpsc, Arc, Mutex}, thread, time::{Duration, Instant}};

mod gpio;
mod calc;
mod api;
mod ws;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct CalcData {
  total_distance: f64,
  total_time_ms: u128,
  speed: f64,
}

const BELT_LEN: f64 = 6.0;

fn main() {
  let (from_gpio, to_calc) = mpsc::channel::<()>();
  let (from_calc, to_ws) = mpsc::channel::<CalcData>();

  let total_time = Arc::new(Mutex::new(0.0f64));
  let total_time_clone = Arc::clone(&total_time);

  let start_time = Arc::new(Mutex::new(Instant::now()));
  let start_time_clone = Arc::clone(&start_time);

  let calculated_data: Arc<Mutex<Vec<CalcData>>> = Arc::new(Mutex::new(Vec::new()));
  let calculated_data_clone = Arc::clone(&calculated_data);
  let calculated_data_clone2 = Arc::clone(&calculated_data);

  let websocket_connected = Arc::new(Mutex::new(false));
  let websocket_connected_copy = Arc::clone(&websocket_connected);

  thread::spawn(|| gpio::run(from_gpio, calculated_data, start_time, total_time));
  thread::spawn(|| calc::run(to_calc, from_calc, calculated_data_clone, start_time_clone, total_time_clone, websocket_connected));
  thread::spawn(|| api::run(calculated_data_clone2));
  thread::spawn(|| ws::run(to_ws, websocket_connected_copy));

  loop {
    thread::sleep(Duration::from_secs(100));
  }
}
