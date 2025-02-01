use std::{sync::mpsc::Sender, thread, time::Duration};

// use gpio::GpioIn;

// const GPIO_PIN_TRIGGER: u16 = 23;
// const GPIO_PIN_ECHO: u16 = 24;

pub fn run(tx: Sender<()>) {
  // let mut gpio_trigger =
  //   gpio::sysfs::SysFsGpioInput::open(GPIO_PIN_TRIGGER)
  //   .expect("Could not open trigger gpio pin");

  // let mut gpio_echo =
  //   gpio::sysfs::SysFsGpioInput::open(GPIO_PIN_ECHO)
  //   .expect("Could not open echo gpio pin");

  loop {
    // let value_trigger = gpio_trigger.read_value().expect("Could not read gpio pin");
    // let value_echo = gpio_echo.read_value().expect("Could not read gpio pin");
    // println!("{value_trigger:?}, {value_echo:?}");
    thread::sleep(Duration::from_secs(1));
    tx.send(()).unwrap();
  }
}