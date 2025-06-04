use crate::firmata::*;
use crate::hardware::{PinManager, PinManagerExt};
use log::debug;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct FirmataParser {
    socket: TcpStream,
    pm: PinManager,
}

impl FirmataParser {
    pub fn new(socket: TcpStream) -> FirmataParser {
        FirmataParser {
            socket,
            pm: PinManager::new(),
        }
    }

    pub async fn read(&mut self, size: usize) -> Result<Vec<u8>, FirmataError> {
        let mut buf: Vec<u8> = vec![0; size];
        match self.socket.read_exact(&mut buf).await {
            Ok(0) => {
                // Connection closed by the peer
                debug!("Connection closed by peer.");
                Err(FirmataError::ConnectionClosed)
            }
            Ok(_) => {
                debug!("Received data: {:02X?}", buf);
                Ok(buf)
            }
            Err(e) => {
                // An I/O error occurred
                eprintln!("Connection error while reading from socket: {}", e);
                Err(FirmataError::ConnectionError(e))
            }
        }
    }

    /// Runs the parser, reading from the socket and processing Firmata messages.
    pub async fn run(mut self) -> Result<(), FirmataError> {
        loop {
            match self.read(1).await?[0] {
                // TODO: parse other Firmata messages
                // REPORT_PROTOCOL_VERSION => self.handle_protocol_version(&buf),
                // ANALOG_MESSAGE..=ANALOG_MESSAGE_BOUND => self.handle_analog_message(&buf),
                START_SYSEX => self.handle_sysex_message().await?,
                DIGITAL_MESSAGE => self.handle_digital_message().await?,
                SYSTEM_RESET => SYSTEM_RESET,
                x => {
                    eprintln!("FirmataParser: skipping unexpected message type: {:02X?}", x);
                    x
                }
            };
        }
    }

    pub async fn handle_sysex_message(&mut self) -> Result<u8, FirmataError> {
        Ok(START_SYSEX)
    }
    
    pub async fn handle_digital_message(&mut self) -> Result<u8, FirmataError> {
        let buf = self.read(3).await?;
        let pin = buf[0] & 0x0F;
        let lsb = buf[1] & 0x7F;
        let msb = buf[2] & 0x7F;
        let value = (msb << 7) | lsb;
        let digital = value != 0;
        println!("Set pin {} to {}", pin, digital);
        self.pm.set_pin(pin, digital)?;
        Ok(DIGITAL_MESSAGE)
    }
}
