//! Defines Firmata message parser from the client.

use crate::firmata::*;
use crate::hardware::{PinManager, PinManagerExt};
use log::{debug, trace};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct FirmataParser {
    socket: TcpStream, // the TCP socket to read from
    pm: PinManager,    // the pin manager
}

impl FirmataParser {
    pub fn new(socket: TcpStream, pm: PinManager) -> FirmataParser {
        FirmataParser { socket, pm }
    }

    /// Read `size` bytes from the socket.
    async fn read(&mut self, size: usize) -> Result<Vec<u8>, FirmataError> {
        let mut buf: Vec<u8> = vec![0; size];
        if let Err(e) = self.socket.read_exact(&mut buf).await {
            eprintln!("Connection error while reading from socket: {}", e);
            return Err(FirmataError::ConnectionError(e));
        }

        trace!("Received data: {:02X?}", buf);
        Ok(buf)
    }

    /// Read bytes from the socket until SYSEX_END is received.
    async fn read_sysex_data(&mut self) -> Result<Vec<u8>, FirmataError> {
        let mut buf: Vec<u8> = vec![];
        loop {
            // Read until END_SYSEX.
            let mut byte = [0];
            if let Err(e) = self.socket.read_exact(&mut byte).await {
                eprintln!("Connection error while reading from socket: {}", e);
                return Err(FirmataError::ConnectionError(e));
            }
            if byte[0] == END_SYSEX {
                trace!("Received data: {:02X?}", buf);
                return Ok(buf);
            }
            buf.push(byte[0]);
        }
    }

    /// Writes a `buf` vector of data into the socket.
    async fn write(&mut self, buf: Vec<u8>) -> Result<Vec<u8>, FirmataError> {
        if let Err(e) = self.socket.write_all(buf.as_slice()).await {
            eprintln!("Connection error while reading from socket: {}", e);
            return Err(FirmataError::ConnectionError(e));
        }
        Ok(buf)
    }

    /// Runs the parser, reading from the socket and processing Firmata messages.
    pub async fn run(mut self) -> Result<(), FirmataError> {
        loop {
            let command = self.read(1).await?[0];
            debug!("New command received: {:02X?}", command);
            match command {
                DIGITAL_MESSAGE => self.handle_digital_message().await?,
                SYSTEM_RESET => (),
                START_SYSEX => {
                    let sysex_command = self.read(1).await?[0];
                    debug!("New sysex command received: {:02X?}", sysex_command);
                    match sysex_command {
                        REPORT_FIRMWARE => self.handle_firmware_report().await?,
                        CAPABILITY_QUERY => self.handle_capability_query().await?,
                        ANALOG_MAPPING_QUERY => self.handle_analog_mapping_response().await?,
                        END_SYSEX => (),
                        x => eprintln!("FirmataParser: skipping unexpected sysex command: {:02X?}", x),
                    }
                }
                x => eprintln!("FirmataParser: skipping unexpected message type: {:02X?}", x),
            };
        }
    }

    async fn handle_firmware_report(&mut self) -> Result<(), FirmataError> {
        let buf = self.read_sysex_data().await?;
        trace!("handle_firmware_report: {:02X?}", buf);
        let mut report = vec![START_SYSEX, REPORT_FIRMWARE, 1, 0];
        report.extend_from_slice(format!("FirmataBridge (using {})", self.pm.get_name()).as_bytes());
        report.push(END_SYSEX);
        debug!("Send firmware report: {:?}", report);
        self.write(report).await?;
        Ok(())
    }

    async fn handle_capability_query(&mut self) -> Result<(), FirmataError> {
        let buf = self.read_sysex_data().await?;
        trace!("handle_firmware_report: {:02X?}", buf);
        let mut capabilities = vec![START_SYSEX, CAPABILITY_RESPONSE];
        capabilities.extend(self.pm.get_capabilities());
        capabilities.push(END_SYSEX);
        debug!("Send capabilities: {:?}", capabilities);
        self.write(capabilities).await?;
        Ok(())
    }

    async fn handle_analog_mapping_response(&mut self) -> Result<(), FirmataError> {
        let buf = self.read_sysex_data().await?;
        trace!("handle_analog_mapping_response: {:02X?}", buf);
        let mut analog_mapping = vec![START_SYSEX, ANALOG_MAPPING_RESPONSE];
        analog_mapping.extend(self.pm.get_analog_mapping());
        analog_mapping.push(END_SYSEX);
        debug!("Send analog mapping: {:?}", analog_mapping);
        self.write(analog_mapping).await?;
        Ok(())
    }

    async fn handle_digital_message(&mut self) -> Result<(), FirmataError> {
        let buf = self.read(2).await?;
        trace!("handle_digital_message: {:02X?}", buf);
        let pin = buf[0];
        let value = buf[1] == 1;
        debug!("Set pin {} to {}", pin, value);
        self.pm.set_pin(pin, value)
    }
}
