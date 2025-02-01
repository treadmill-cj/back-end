use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

mod gpio;
mod calc;
mod api;
mod ws;

struct CalcData {

}

fn main() {
  let (from_gpio, to_calc) = mpsc::channel::<Duration>();
  let (from_calc, to_ws) = mpsc::channel::<CalcData>();

  let calculated_data: Arc<Mutex<Vec<CalcData>>> = Arc::new(Mutex::new(Vec::new()));
  let calculated_data_clone = Arc::clone(&calculated_data);

  thread::spawn(|| gpio::run(from_gpio));
  thread::spawn(|| calc::run(to_calc, from_calc, calculated_data));
  thread::spawn(|| api::run(calculated_data_clone));
  thread::spawn(|| ws::run(to_ws));

  loop {
    thread::sleep(Duration::from_secs(100));
  }
}
