use std::{sync::mpsc::Sender, thread, time::{Duration, Instant}};

use rppal::gpio::Gpio;

const GPIO_PIN: u8 = 15;

pub fn run(tx: Sender<()>) {
  let mut pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_input_pullup();

  loop {
    while pin.is_high() {} // wait
    tx.send(()).unwrap();
    println!("BAM!!!");
    thread::sleep(Duration::from_millis(200));
    while pin.is_low() {} // wait to reset
    thread::sleep(Duration::from_millis(200));
  }
}