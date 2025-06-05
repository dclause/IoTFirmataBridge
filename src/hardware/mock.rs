// @todo

use crate::firmata::*;
use crate::hardware::PinManagerExt;

#[derive(Clone)]
pub struct MockPinManager;

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
            7 | 8 | 9 | 10 | 11 => {
                // Analog pin
                capabilities.extend_from_slice(&[MODE_ANALOG, RESOLUTION_ANALOG]);
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

impl PinManagerExt for MockPinManager {
    fn new() -> Self {
        MockPinManager
    }

    fn get_name(&self) -> String {
        String::from("MockBoard")
    }
    fn get_capabilities(&self) -> Vec<u8> {
        let mut response = vec![];
        for pin_num in 0..=40 {
            response.extend(self.get_pin_capabilities(pin_num));
        }
        response
    }

    fn get_analog_mapping(&self) -> Vec<u8> {
        let mut response = vec![]; // No analog pin
        for pin_num in 0..=40 {
            match pin_num {
                7 | 8 | 9 | 10 | 11 => response.push(pin_num as u8),
                _ => response.push(SYSEX_REALTIME), // Unsupported pin
            }
        }
        response
    }

    fn set_pin(&self, pin: u8, value: bool) -> Result<(), FirmataError> {
        println!("DefaultPinManager: (virtual) set_pin({}) = {}", pin, value);
        Ok(())
    }
}
