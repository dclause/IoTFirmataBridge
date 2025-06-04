use crate::firmata::FirmataError;
#[cfg(feature = "raspberry")]
mod raspberry;
#[cfg(feature = "raspberry")]
pub use crate::hardware::raspberry::RaspiPinManager as PinManager;

#[cfg(feature = "jetson")]
mod jetson;
#[cfg(feature = "jetson")]
pub use crate::hardware::jetson::JetsonPinManager as PinManager;

#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use crate::hardware::mock::MockPinManager as PinManager;

// Définition du trait PinManager
pub trait PinManagerExt: Send + Sync + 'static {
    fn new() -> Self where Self: Sized;
    fn set_pin(&self, pin: u8, value: bool) -> Result<(), FirmataError>;
}