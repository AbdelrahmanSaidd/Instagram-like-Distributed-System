use std::net::UdpSocket;
use std::io;

use std::thread;
use std::time::Duration;


fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8083";
    let socket = UdpSocket::bind(server_address)?;
    
    let prev_server_address = "127.0.0.1:8080";
    let next_server_address = "127.0.0.1:8082";

    let mut turn = 2;
    let mut counter = 2; 

    let mut counter_as_str;
    let mut update = "";
    let mut port;

    println!("UDP Server {} is listening on {}", turn, server_address);

    let mut buffer = [0u8; 1024];

    let mut active_servers_as_str;
    let mut turn_as_str;
    let mut active_servers = 3;
    let mut remainging_to_fail = 3;
    loop {
        if remainging_to_fail == 0{
            println!("I will fail now");
            port = 3;
            active_servers -= 1;
            // broadcasting the server's failure ==> -turn
            update = "-";
            turn_as_str = update.to_string() + &turn.to_string();
            let turn_data = turn_as_str.as_bytes();
            println!("Informing the servers of my failure at port {} for turn {}!", port, turn);
            socket.send_to(turn_data, prev_server_address)?;
            socket.send_to(turn_data, next_server_address)?;
            println!("");
            turn = -1;
            
            let sleep_duration = Duration::from_secs(5);
            thread::sleep(sleep_duration);
            
            println!("I am back now");
            remainging_to_fail = 100;
            port = 4;
            update = "*";
            let update_data = update.as_bytes();

            println!("Informing the servers I am back at port {}!", port);
            socket.send_to(update_data, prev_server_address)?;
            socket.send_to(update_data, next_server_address)?;


        }
        match socket.recv_from(&mut buffer) {
            Ok((n, sender_address)) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                // functionality: received a request from a PEER SERVER => check on PORT 2
                // to be adjusted to check for a PORT number when testing on a network scale
                if sender_address.to_string() == next_server_address || sender_address.to_string() == prev_server_address{
                    
                    //handling receiving at port 2
                    // Initialize a flag to indicate whether a '+' is in the variable.
                    let started_with_plus_flag: bool = message.starts_with('+');
                    // Parse the INTEGER and store it in a new variable.
                    if started_with_plus_flag{
                    let received_counter: i32 = message
                        .trim_start_matches('+')
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Failed to parse the INTEGER");
                            std::process::exit(1);
                        });
                    if turn != -1{
                        println!("I received a new counter: {}", received_counter);
                    }
                    //println!("update: {}", update);
                    //println!("counter: {}", received_counter);
                    counter = received_counter;
                    println!("");
                    }

                    // handling receiving at port 3 - functionality: server failed
                    // Initialize a flag to indicate whether a '+' is in the variable.
                    let sever_failed_flag: bool = message.starts_with('-');
                    // Parse the INTEGER and store it in a new variable.
                    
                    if sever_failed_flag{
                        let server_off_turn: i32 = message
                        .trim_start_matches('-')
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Failed to parse the INTEGER");
                            std::process::exit(1);
                        });
                    
                        active_servers -= 1;
                        if turn > server_off_turn{
                            turn -= (turn - server_off_turn);
                        }
                        println!("I got a new turn: {}", turn);
                        counter = 1;
                        println!("Current active servers: {}", active_servers);
                        println!("");
                      
                    }
                 
                    // handling receiving at port 4 - functionality: server is recovered/joining
                    // Initialize a flag to indicate whether a '+' is in the variable.
                    let server_joined_flag: bool = message.starts_with('*');

                    // send back the new number of servers (to indicate the joining server's turn)
                    if server_joined_flag{
                    active_servers += 1;
                    port = 5;
                    update = "N";
                    active_servers_as_str = update.to_string() + &active_servers.to_string();
                    let active_servers_data = active_servers_as_str.as_bytes();
                    println!("I am sending back the new active servers number: {}", active_servers);
                    socket.send_to(active_servers_data, sender_address)?;
                    println!("");
                    }

                    
                    // handling receiving at port 5 - functionality: receiving the updated number of active servers (to know my turn)
                    // Initialize a flag to indicate whether a 'N' is in the variable.
                    let new_number_flag: bool = message.starts_with('N');
                    // Parse the INTEGER and store it in a new variable.
                    if new_number_flag{
                    let new_servers_num: i32 = message
                        .trim_start_matches('N')
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Failed to parse the INTEGER");
                            std::process::exit(1);
                        });
                    
                    
                        active_servers = new_servers_num;
                        turn = new_servers_num;
                        println!("I got a new turn after rejoining: {}", turn);

                        println!("Current active servers: {}", active_servers);
                        println!("");
                    }


                }
                else if turn != -1{ // turn -1 refers to the case when the server is recovering from a failure and is waitig to be assigned to a new turn
                    // handling receiving at port 1
                    // functionality: received a request from a CLIENT => check on PORT 1
                    // to be adjusted to check for a PORT number when testing on a network scale
                    println!("Server 2: Received a request from {}: {}", sender_address, message);
                    if counter == turn{
                        println!("This is my turn");
                        remainging_to_fail -= 1; 
                        port = 1;
                        let mut received_number: i32 = message.parse().unwrap_or(0);                                    
                        received_number += 1;
                        let data_to_send = received_number.to_string();
                        socket.send_to(data_to_send.as_bytes(), sender_address.to_string())?;
                        counter = (counter%active_servers) + 1;
                        update = "+";
                        counter_as_str = update.to_string() + &counter.to_string();
                        let counter_data = counter_as_str.as_bytes();
                        println!("Sending the new counter ({}) to the servers!", counter);
                        socket.send_to(counter_data, prev_server_address)?;
                        socket.send_to(counter_data, next_server_address)?;
                        println!("");

                    }
                    else{
                        println!("This is NOT my turn");
                        //println!("Adding the request to a buffer!");
                    }
                }
            }
            Err(e) => {
                eprintln!("Server 2: Error receiving data: {}", e);
            }
        }
    }
}
