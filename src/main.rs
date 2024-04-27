mod structures;
mod serialisers;
mod utils;

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Read, net::TcpStream};

use crate::structures::{BitcoinMessage, NetAddr, VersionPayload};
use crate::serialisers::{construct_complete_message, serialise_version_message};






fn main() {
    println!("Hello, world TCP connection Test 1");

    let addr_recv = NetAddr {
        services: 0,
        ip_v6_4: "::ffff:2d90:70d0".parse().unwrap(),
        port: 8333,
    };

    let addr_from = NetAddr {
        services: 0,
        ip_v6_4: "::ffff:54cb:4f70".parse().unwrap(),
        port: 8333,
    };

    let version_message = VersionPayload {
        version: 70015, // current protocol version
        services: 1,    // node is a full node
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        addr_recv, // receiving node's address
        addr_from, // sending node's address
        nonce: rand::random::<u64>(),                                  // random nonce
        user_agent: b"/Satoshi:0.7.2/".to_vec(),                       // user agent
        start_height: 0,                                               // block height
        relay: true, // node will relay transactions
    };



    // Serialize the struct into bytes
    let serialized_version_message = match serialise_version_message(&version_message) {
        Ok(message) => message,
        Err(e) => {
            println!("Failed to serialize version message: {}", e);
            return;
        }
    };

    // Construct the complete message
    let complete_message = construct_complete_message(serialized_version_message);


 

    //  The idea here is to create a socket connection to the bitcoin node

    let mut stream = match TcpStream::connect("45.144.112.208:8333") {
        Ok(stream) => {
            println!("Connected to the node!");
            stream
        }
        Err(e) => {
            println!("Couldnt Connect to the node: {}", e);
            return;
        }
    };

    // now send the message 



// Send the message
match stream.write_all(&complete_message) {
    Ok(_) => println!("Message sent!"),
    Err(e) => {
        println!("Failed to send message: {}", e);
        return;
    }
}

// Listen for a response
let mut header_buffer = [0; 24];
match stream.read_exact(&mut header_buffer) {
    Ok(_) => {
        let length = u32::from_le_bytes((&header_buffer[16..20]).try_into().unwrap()) as usize;

        // Read the payload
        let mut payload_buffer = vec![0; length];
        match stream.read_exact(&mut payload_buffer) {
            Ok(_) => {
                let message = BitcoinMessage {
                    magic: u32::from_le_bytes((&header_buffer[0..4]).try_into().unwrap()),
                    command: String::from_utf8((&header_buffer[4..16]).try_into().unwrap()).unwrap().trim_end_matches('\0').to_string(),
                    length: length as u32,
                    payload: payload_buffer,
                };

                println!("BitcoinMessage: {:?}", message);
            }
            Err(e) => {
                println!("Failed to read payload: {}", e);
                return;
            }
        }
    }
    Err(e) => {
        println!("Failed to read header: {}", e);
        return;
    }
}

}
