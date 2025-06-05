// @todo

use crate::firmata::*;
use crate::hardware::PinManagerExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MockPinManager {
    pins: Arc<Mutex<HashMap<u8, usize>>>,
}

impl MockPinManager {
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
            5 | 6 | 10 | 11 | 21 | 22 => {
                // Analog pin
                capabilities.extend_from_slice(&[MODE_ANALOG, RESOLUTION_ANALOG]);
                // SPI0
                capabilities.extend_from_slice(&[MODE_SPI, RESOLUTION_SPI]);
            }
            8 | 9 | 18 | 19 | 28 | 29 => {
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

    /// Updates the pin value. Returns true/false if the new value is different from the prev one.
    fn set_state(&self, pin: u8, value: usize) -> bool {
        let mut pins = self.pins.lock().unwrap();
        if !pins.contains_key(&pin) {
            pins.insert(pin, value);
            return true;
        }

        match pins.insert(pin, value) {
            None => true,
            Some(prev) => prev != value,
        }
    }
}

impl PinManagerExt for MockPinManager {
    fn new() -> Self {
        MockPinManager {
            pins: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_name(&self) -> String {
        String::from("MockBoard")
    }
    fn get_capabilities(&self) -> Vec<u8> {
        let mut response = vec![];
        for pin_num in 0..=30 {
            response.extend(self.get_pin_capabilities(pin_num));
        }
        response
    }

    fn get_analog_mapping(&self) -> Vec<u8> {
        let mut response = vec![]; // No analog pin
        for pin_num in 0..=40 {
            match pin_num {
                5 | 6 | 10 | 11 | 21 | 22 => response.push(pin_num as u8),
                _ => response.push(SYSEX_REALTIME), // Unsupported pin
            }
        }
        response
    }

    fn set_pin_mode(&self, pin: u8, mode: u8) -> Result<(), FirmataError> {
        println!("DefaultPinManager: (virtual) set pin {} to mode {}", pin, mode);
        Ok(())
    }

    fn set_digital_pin(&self, pin: u8, value: bool) -> Result<(), FirmataError> {
        if self.set_state(pin, value as usize) != value {
            println!("DefaultPinManager: (virtual) set digital pin {} = {}", pin, value);
        }
        Ok(())
    }

    fn set_analog_pin(&self, pin: u8, value: usize) -> Result<(), FirmataError> {
        if self.set_state(pin, value) {
            println!("DefaultPinManager: (virtual) set analog pin {} = {}", pin, value);
        }
        Ok(())
    }

    fn send_i2c_data(&self, address: u16, data: Vec<u16>) -> Result<(), FirmataError> {
        println!("DefaultPinManager: (virtual) send i2c data (address={}) data={:?}", address, data);
        Ok(())
    }
}
