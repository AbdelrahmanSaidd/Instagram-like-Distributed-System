use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Define the addresses of the three servers
    let server_addresses = [
        "127.0.0.1:8085"
    ];

    // Create a UDP socket for the client.
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Message to send to the servers.
    let message = "Hello, servers!";

    for &server_address in &server_addresses {
        // Send the message to each server.
        socket.send_to(message.as_bytes(), server_address)?;

        println!("Message sent to server {}: {}", server_address, message);

        // Receive a response from the server.
        let mut response_buffer = [0u8; 1024];
        let (n, _) = socket.recv_from(&mut response_buffer)?;
        let response_message = String::from_utf8_lossy(&response_buffer[..n]);

        println!("Received response from server {}: {}", server_address, response_message);
    }

    Ok(())
}
