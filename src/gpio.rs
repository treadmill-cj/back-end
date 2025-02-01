use std::{sync::mpsc::Sender, thread, time::{Duration, Instant}};

use rppal::gpio::Gpio;

const GPIO_PIN: u8 = 24;

pub fn run(tx: Sender<()>) {
  let mut pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_input();

  loop {
    println!("{:?}", pin.is_high());

    thread::sleep(Duration::from_millis(10));
    tx.send(()).unwrap();
  }
}