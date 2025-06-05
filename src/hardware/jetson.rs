// @todo

use super::PinManagerExt;

#[derive(Clone)]
pub struct JetsonPinManager;

impl PinManagerExt for JetsonPinManager {
    fn new() -> Self {
        JetsonPinManager
    }

    fn set_digital_pin(&self, pin: u8, value: bool) {
        // TODO: Implémenter accès GPIO spécifique Jetson ici
        println!("Jetson: set pin {} to {}", pin, value);
    }
}
