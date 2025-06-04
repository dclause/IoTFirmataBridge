#[cfg(all(feature = "raspberry", feature = "jetson"))]
compile_error!("Features 'raspberry' and 'jetson' are mutually exclusive. Please enable only one.");
mod firmata;
mod hardware;

use crate::firmata::*;
use crate::hardware::{PinManager, PinManagerExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:3030").await?;
    println!("Firmata TCP server listening on port 3030");

    let pin_manager = PinManager::new();

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let pm = pin_manager.clone();

        tokio::spawn(async move {
            FirmataParser::new(socket, pm).run().await?;
            Ok::<(), FirmataError>(())
        });
    }
}
