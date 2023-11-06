use std::net::UdpSocket;
use std::io;

fn main() -> io::Result<()> {
    // Define the addresses of the three servers
    let server_addresses = [
        "127.0.0.1:8080",
        "127.0.0.1:8083",
        "127.0.0.1:8082",
    ];

    // Create a UDP socket for the client.
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Message to send to the servers.
    let data = 100;
    let message = data.to_string();

    let mut i_loop = 0;
    loop{
    println!("iteration: {}", i_loop+1);
    for &server_address in &server_addresses {
        // Send the message to each server.
        socket.send_to(message.as_bytes(), server_address)?;

        println!("Message sent to server {}: {}", server_address, message);
    }
    
    let mut buffer = [0u8; 1024];
    match socket.recv_from(&mut buffer) {
        Ok((n, sender_address)) => {
            let message = String::from_utf8_lossy(&buffer[..n]);
            println!("Client 1: Received a request from {}: {}", sender_address, message);
        }
        Err(e) => {
            eprintln!("Client 1: Error receiving data: {}", e);
        }
    }         
    i_loop += 1;
    if i_loop > 20{
        break;
    }
}         
    Ok(())
}
