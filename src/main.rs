#[cfg(all(feature = "raspberry", feature = "jetson"))]
compile_error!("Features 'raspberry' and 'jetson' are mutually exclusive. Please enable only one.");
mod firmata;
mod hardware;

use crate::firmata::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:3030").await?;

    println!("Firmata TCP server listening on port 3030");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            // Create a new parser for each connection, giving it the socket
            let parser = FirmataParser::new(socket);

            // Run the parser's main loop
            if let Err(e) = parser.run().await {
                match e {
                    FirmataError::ConnectionClosed => {
                        println!("Connection with {} closed.", addr);
                    }
                    _ => {
                        eprintln!("Error handling connection from {}: {}", addr, e);
                    }
                }
            }

            Ok::<(), FirmataError>(())
        });
    }
}
