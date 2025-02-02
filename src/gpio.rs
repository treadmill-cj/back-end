use std::{sync::{mpsc::Sender, Arc, Mutex}, thread, time::{Duration, Instant}};

#[cfg(target_os="linux")]
use rppal::gpio::Gpio;

use crate::CalcData;

const GPIO_PIN: u8 = 14;
const GPIO_PIN_START: u8 = 15;
const GPIO_PIN_STOP: u8 = 18;

#[cfg(target_os="linux")]
pub fn run(tx: Sender<()>, calculated_data: Arc<Mutex<Vec<CalcData>>>, time: Arc<Mutex<Instant>>, dist: Arc<Mutex<f64>>) {
  let mut pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_input_pullup();
  let mut pin_start = Gpio::new().unwrap().get(GPIO_PIN_START).unwrap().into_input_pullup();
  let mut pin_stop = Gpio::new().unwrap().get(GPIO_PIN_STOP).unwrap().into_input_pullup();

  loop {
    {
      calculated_data.lock().unwrap().clear(); // reset everything
      *time.lock().unwrap() = Instant::now();
      *dist.lock().unwrap() = 0.0;
    }
    while pin_start.is_high() {} // wait until it is started
    println!("start");
  
    'a: loop {
      while pin.is_high() { // wait
        if pin_stop.is_low() {
          println!("stop");
          break 'a;
        }
      }
      println!("Bam!");
      tx.send(()).unwrap();
      thread::sleep(Duration::from_millis(200));
      while pin.is_low() {} // wait to reset
      thread::sleep(Duration::from_millis(200));
    }
  }
}

#[cfg(not(target_os="linux"))]
pub fn run(tx: Sender<()>, calculated_data: Arc<Mutex<Vec<CalcData>>>, time: Arc<Mutex<Instant>>, dist: Arc<Mutex<f64>>) {
  loop {
    {
      calculated_data.lock().unwrap().clear(); // reset everything
      *time.lock().unwrap() = Instant::now();
      *dist.lock().unwrap() = 0.0;
    }
    let mut values = 10;
    'a: loop {
      if values == 0 {
        break 'a;
      } values -= 1;
      thread::sleep(Duration::from_secs(1));
      tx.send(()).unwrap();
    }
  }
}