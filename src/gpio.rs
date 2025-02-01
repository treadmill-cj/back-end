use std::{sync::mpsc::Sender, thread, time::{Duration, Instant}};

use rppal::gpio::Gpio;

const GPIO_PIN_TRIGGER: u8 = 23;
const GPIO_PIN_ECHO: u8 = 24;

pub fn run(tx: Sender<()>) {
  let mut trigger = Gpio::new().unwrap().get(GPIO_PIN_TRIGGER).unwrap().into_output();
  let mut echo = Gpio::new().unwrap().get(GPIO_PIN_ECHO).unwrap().into_input();

  loop {
    trigger.set_low();
    thread::sleep(Duration::from_micros(5));
    trigger.set_high();
    thread::sleep(Duration::from_micros(10));
    trigger.set_low();
    println!("triggered");
    let now = Instant::now();
    for _ in 0..50 {
      println!("{}", echo.is_high());
    }
    let time_elapsed = now.elapsed();

    println!("{:?}", time_elapsed);

    thread::sleep(Duration::from_secs(1));
    tx.send(()).unwrap();
  }
}