// @todo

use crate::firmata::FirmataError;
use crate::hardware::PinManagerExt;

#[derive(Clone)]
pub struct MockPinManager;

impl PinManagerExt for MockPinManager {
    fn new() -> Self {
        MockPinManager
    }

    fn set_pin(&self, pin: u8, value: bool) -> Result<(), FirmataError> {
        println!("DefaultPinManager: (virtual) set_pin({}) = {}", pin, value);
        Ok(())
    }
}
