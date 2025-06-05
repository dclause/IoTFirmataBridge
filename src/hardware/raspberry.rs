// Implémentation Raspberry Pi (rppal)

use super::PinManagerExt;
use crate::firmata::{
    MODE_I2C, MODE_INPUT, MODE_OUTPUT, MODE_PWM, MODE_SERVO, MODE_SPI, RESOLUTION_DIGITAL, RESOLUTION_I2C, RESOLUTION_PWM, RESOLUTION_SERVO,
    RESOLUTION_SPI, SYSEX_REALTIME,
};
use parking_lot::Mutex;
use rppal::gpio::{Gpio, OutputPin};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct RaspiPinManager {
    gpio: Arc<Mutex<Gpio>>,
    pins: Arc<Mutex<HashMap<u8, OutputPin>>>,
}

impl RaspiPinManager {
    fn get_pin_capabilities(&self, bcm_pin: u8) -> Vec<u8> {
        let mut capabilities = Vec::new();

        // All pins support INPUT and OUTPUT
        capabilities.extend_from_slice(&[MODE_INPUT, RESOLUTION_DIGITAL]);
        capabilities.extend_from_slice(&[MODE_OUTPUT, RESOLUTION_DIGITAL]);

        // Add other capabilities based on BCM pin number
        match bcm_pin {
            2 | 3 => {
                // I2C1 SDA, SCL
                capabilities.extend_from_slice(&[MODE_I2C, RESOLUTION_I2C]);
            }
            7 | 8 | 9 | 10 | 11 => {
                // SPI0
                capabilities.extend_from_slice(&[MODE_SPI, RESOLUTION_SPI]);
            }
            12 | 13 | 18 | 19 | 32 | 33 | 35 | 36 | 38 | 40 => {
                // Hardware PWM pins
                capabilities.extend_from_slice(&[MODE_PWM, RESOLUTION_PWM]);
                capabilities.extend_from_slice(&[MODE_SERVO, RESOLUTION_SERVO]);
            }
            // Add cases for other special function pins if necessary
            // e.g., UART pins if you implement Firmata Serial
            _ => {
                // Just general INPUT/OUTPUT
            }
        }

        capabilities.push(SYSEX_REALTIME); // End of capabilities for this pin
        capabilities
    }
}

impl PinManagerExt for RaspiPinManager {
    fn new() -> Self {
        Self {
            gpio: Arc::new(Mutex::new(Gpio::new().expect("Failed to initialize GPIO"))),
            pins: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn set_digital_pin(&self, pin: u8, value: bool) {
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

    fn get_capabilities(&self) -> Vec<u8> {
        let mut response = vec![];
        for pin_num in 0..=40 {
            response.extend(self.get_pin_capabilities(pin_num));
        }
        response
    }

    fn get_analog_mapping(&self) -> Vec<u8> {
        vec![SYSEX_REALTIME; 40] // No analog pin
    }
}
