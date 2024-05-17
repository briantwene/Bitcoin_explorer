

use std::error::Error;
use std::io::{Cursor, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Read, net::TcpStream};


use crate::serialisers::{bitcoin_checksum, construct_complete_message, serialise_version_message};
use crate::structures::{BitcoinMessage, Command, NetAddr, VersionPayload};
use crate::utils::BITCOIN_MAGIC;

pub struct Connection {
    stream: Option<TcpStream>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection { stream: None }
    }

    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect("52.57.53.177:8333")?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn handshake(&mut self) -> Result<(), Box<dyn Error>> {
        let stream = match &mut self.stream {
            Some(stream) => stream,
            None => return Err("Not connected to a node".into()),
        };

        //prep payload
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
            addr_recv,                               // receiving node's address
            addr_from,                               // sending node's address
            nonce: rand::random::<u64>(),            // random nonce
            user_agent: b"/Satoshi:0.7.2/".to_vec(), // user agent
            start_height: 0,                         // block height
            relay: true,                             // node will relay transactions
        };

        let verack_message = BitcoinMessage {
            magic: 0xd9b4bef9,
            command: "verack".to_string(),
            length: 0,
            payload: vec![],
        };

        // Serialize the struct into bytes
        let serialized_version_message = match serialise_version_message(&version_message) {
            Ok(message) => message,
            Err(e) => {
                println!("Failed to serialize version message: {}", e);
                return Err("Failed to serialize version message".into());
            }
        };

        // Construct the complete message
        let complete_message =
            construct_complete_message(Command::Version, serialized_version_message);

        stream.write_all(&complete_message)?;

        // Listen for a response
        let mut header_buffer = [0; 24];
        match stream.read_exact(&mut header_buffer) {
            Ok(_) => {
                let length =
                    u32::from_le_bytes((&header_buffer[16..20]).try_into().unwrap()) as usize;

                // Read the payload
                let mut payload_buffer = vec![0; length];
                match stream.read_exact(&mut payload_buffer) {
                    Ok(_) => {
                        let message = BitcoinMessage {
                            magic: u32::from_le_bytes((&header_buffer[0..4]).try_into().unwrap()),
                            command: String::from_utf8((&header_buffer[4..16]).try_into().unwrap())
                                .unwrap()
                                .trim_end_matches('\0')
                                .to_string(),
                            length: length as u32,
                            payload: payload_buffer,
                        };

                        println!("BitcoinMessage: {:?}", message);
                    }
                    Err(e) => {
                        println!("Failed to read payload: {}", e);
                        return Err("Failed to read payload".into());
                    }
                }
            }
            Err(e) => {
                println!("Failed to read header: {}", e);
                return Err("Failed to read header".into());
            }
        }



        let verack_bytes = construct_complete_message(Command::Verack, vec![]);

        match stream.write_all(&verack_bytes) {
            Ok(_) => println!("Verack message sent!"),
            Err(e) => {
                println!("Failed to send verack message: {}", e);
                return Err("Failed to send verack message".into());
            }
        }
        Ok(())
    }

    pub fn handle_stream(&mut self) -> Result<(), Box<dyn Error>> {

        let stream = match &mut self.stream {
            Some(stream) => stream,
            None => return Err("Not connected to a node".into()),
        };

      
        
        loop {
            // create buffer
            let mut buffer: Vec<u8> = vec![0; 24];

            let num_bytes = stream.read(&mut buffer).unwrap();

            if num_bytes >= 24 {
                let magic_bytes = &buffer[0..4];
                let command_bytes = &buffer[4..16];
                let length_bytes = &buffer[16..20];
                let checksum_bytes = &buffer[20..24];

                if magic_bytes == BITCOIN_MAGIC {
                    println!("Magic number matched");

                    let command = String::from_utf8_lossy(command_bytes)
                        .trim_end_matches('\0')
                        .to_string();
                    let length = u32::from_le_bytes(length_bytes.try_into().unwrap());
                    let checksum = u32::from_le_bytes(checksum_bytes.try_into().unwrap());

                    println!("Command: {}", command);
                    println!("Length: {}", length);
                    println!("Checksum: {}\n\n", checksum);

                    let mut payload: Vec<u8> = vec![0; length as usize];
                    stream.read_exact(&mut payload).unwrap();

                    if command == "ping" {
                        let nonce = u64::from_le_bytes(payload[0..8].try_into().unwrap());
                        handle_ping(nonce, stream).unwrap();
                    } else if command == "inv" {
                        handle_inv(payload, stream).unwrap();
 
                    } else if command == "block" {
                        handle_block(payload).unwrap()
                    }
                }
            }
        }

        Ok(())
    }



}


fn handle_ping(nonce: u64, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {


    let pong_command = Command::Pong.as_bytes();
    let pong_length = 8u32.to_le_bytes();
    let pong_checksum = bitcoin_checksum(&nonce.to_le_bytes());

    let mut pong_message = Vec::new();
    pong_message.extend_from_slice(&BITCOIN_MAGIC);
    pong_message.extend_from_slice(&pong_command);
    pong_message.extend_from_slice(&pong_length);
    pong_message.extend_from_slice(&pong_checksum);
    pong_message.extend_from_slice(&nonce.to_le_bytes());

    stream.write_all(&pong_message).unwrap();
    println!("Sent pong message with nonce {}", nonce);


    Ok(())

}

fn handle_inv(payload: Vec<u8>, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let first_byte = payload[0];
    let (count, offset) = match first_byte {
        value if value < 0xFD => (value as u64, 1),
        0xFD => (u16::from_le_bytes(payload[1..3].try_into().unwrap()) as u64, 3),
        0xFE => (u32::from_le_bytes(payload[1..5].try_into().unwrap()) as u64, 5),
        0xFF => (u64::from_le_bytes(payload[1..9].try_into().unwrap()), 9),
        _ => return Err("Invalid count".into()),
    };

    println!("Count: {}", count);

    let inv_vectors: Vec<&[u8]> = payload[offset..].chunks(36).collect();

    //println!("Inventory vectors: {:?}", inv_vectors);

    for inv_vector in inv_vectors {
  
  

        // Read the type as a 4-byte array
        let inv_type_bytes = &inv_vector[0..4];

        // Convert the type to a u32
        let inv_type = u32::from_le_bytes(inv_type_bytes.try_into().unwrap());

        // Check if the type is a block get its hash and then send a getdata message
        if inv_type == 2 {
            let hash = &inv_vector[4..];
            println!("Block hash: {:?}", hash);
            send_getdata(hash, inv_type, stream).unwrap();
        }

        // Check if the type is a transaction
        // if inv_type == 1 {
        //     let hash = &inv_vector[4..];
        //     println!("Transaction hash: {:?}", hash);
        // }
    }

    Ok(())
}


fn send_getdata(hash: &[u8], inv_type: u32, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let getdata_command = Command::GetData.as_bytes();

    let mut getdata_payload = Vec::new();

    getdata_payload.push(1);

    getdata_payload.extend(&inv_type.to_le_bytes());

    getdata_payload.extend(hash);

    let getdata_length = (getdata_payload.len() as u32).to_le_bytes();
    let getdata_checksum = bitcoin_checksum(&getdata_payload);


    let mut getdata_message = Vec::new();
    getdata_message.extend_from_slice(&BITCOIN_MAGIC);
    getdata_message.extend_from_slice(&getdata_command);
    getdata_message.extend_from_slice(&getdata_length);
    getdata_message.extend_from_slice(&getdata_checksum);
    getdata_message.extend_from_slice(&getdata_payload);

    stream.write_all(&getdata_message).unwrap();
    println!("Sent getdata message for hash {:?}", hash);

    Ok(())
}

fn handle_block(block: Vec<u8>) -> Result<(), Box<dyn Error>> {
    // The block header is the first 80 bytes of the payload
    let header = &block[0..80];

    // The rest of the payload is the transactions
    let transactions = &block[80..];

    // Parse the header
    let version = u32::from_le_bytes(header[0..4].try_into().unwrap());
    let prev_block_hash = &header[4..36];
    let merkle_root = &header[36..68];
    let timestamp = u32::from_le_bytes(header[68..72].try_into().unwrap());
    let bits = u32::from_le_bytes(header[72..76].try_into().unwrap());
    let nonce = u32::from_le_bytes(header[76..80].try_into().unwrap());

    println!("Block header:");
    println!("Version: {}", version);
    println!("Previous block hash: {:?}", prev_block_hash);
    println!("Merkle root: {:?}", merkle_root);
    println!("Timestamp: {}", timestamp);
    println!("Bits: {}", bits);
    println!("Nonce: {}", nonce);

    // TODO: Parse the transactions

    Ok(())
}