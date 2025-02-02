use std::{sync::mpsc::Sender, thread, time::{Duration, Instant}};

use rppal::gpio::Gpio;

const GPIO_PIN: u8 = 15;
const GPIO_PIN_START: u8 = 15;
const GPIO_PIN_STOP: u8 = 15;

pub fn run(tx: Sender<()>, calculated_data: Arc<Mutex<Vec<CalcData>>>) {
  let mut pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_input_pullup();
  let mut pin_start = Gpio::new().unwrap().get(GPIO_PIN_START).unwrap().into_input_pullup();
  let mut pin_stop = Gpio::new().unwrap().get(GPIO_PIN_STOP).unwrap().into_input_pullup();

  loop {
    {
      calculated_data.lock().unwrap().clear(); // reset everything
    }
    while pin_start.is_high() {} // wait until it is started
  
    'a: loop {
      while pin.is_high() { // wait
        if pin_stop.is_low() {break 'a;}
      }
      tx.send(()).unwrap();
      thread::sleep(Duration::from_millis(200));
      while pin.is_low() {} // wait to reset
      thread::sleep(Duration::from_millis(200));
    }
  }
}