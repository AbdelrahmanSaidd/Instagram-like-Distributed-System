use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8085"; // Adjust for each server

    let socket = UdpSocket::bind(server_address)?;

    println!("UDP server is listening on {}", server_address);

    let mut buffer = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((n, client_address)) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Server: Received a message from {}: {}", client_address, message);

                // Send a response back to the client
                let response = "Message received by the server!";
                socket.send_to(response.as_bytes(), client_address)?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
