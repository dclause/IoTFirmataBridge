// Implémentation Raspberry Pi (rppal)

use super::PinManagerExt;
use rppal::gpio::{Gpio, OutputPin};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Clone)]
pub struct RaspiPinManager {
    gpio: Arc<Mutex<Gpio>>,
    pins: Arc<Mutex<HashMap<u8, OutputPin>>>,
}

impl PinManagerExt for RaspiPinManager {
    fn new() -> Self {
        Self {
            gpio: Arc::new(Mutex::new(Gpio::new().expect("Failed to initialize GPIO"))),
            pins: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn set_pin(&self, pin: u8, value: bool) {
        let mut pins = self.pins.lock().unwrap();
        if !pins.contains_key(&pin) {
            let gpio_guard = self.gpio.lock().unwrap();
            let out = gpio_guard.get(pin).expect("Failed to get GPIO pin").into_output();
            pins.insert(pin, out);
        }
        if let Some(p) = pins.get_mut(&pin) {
            p.write(if value { rppal::gpio::Level::High } else { rppal::gpio::Level::Low });
        }
    }
}