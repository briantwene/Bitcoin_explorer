

use sha2::{Digest, Sha256};


use crate::utils::ip_string_to_bytes; // import the function
use crate::structures::{Command, NetAddr, VersionPayload};



pub fn serialise_version_message(version_message: &VersionPayload) -> Result<Vec<u8>, std::net::AddrParseError> {
    let mut message_bytes = vec![];
    let net_addr_from = serialise_network_address(&version_message.addr_from)?;
    let net_addr_recv = serialise_network_address(&version_message.addr_recv)?;

    message_bytes.extend((&version_message.version).to_le_bytes());
    message_bytes.extend((&version_message.services).to_le_bytes());
    message_bytes.extend((&version_message.timestamp).to_le_bytes());
    message_bytes.extend(net_addr_from);
    message_bytes.extend(net_addr_recv);
    message_bytes.extend((&version_message.nonce).to_le_bytes());

    // User agent length (as a u8)
    message_bytes.push(version_message.user_agent.len() as u8);
    message_bytes.extend_from_slice(&version_message.user_agent);

    message_bytes.extend((&version_message.start_height).to_le_bytes());
    message_bytes.push(version_message.relay as u8);

    Ok(message_bytes)
}


fn serialise_network_address(addr: &NetAddr) -> Result<Vec<u8>, std::net::AddrParseError> {
    let mut buf = vec![];

    buf.extend(&addr.services.to_le_bytes());
    let ip_bytes = ip_string_to_bytes(&addr.ip_v6_4)?;
    buf.extend(&ip_bytes);
    buf.extend(&addr.port.to_le_bytes());

    Ok(buf)
}

pub fn bitcoin_checksum(payload: &[u8]) -> [u8; 4] {
    let first_hash = Sha256::digest(payload);
    let second_hash = Sha256::digest(&first_hash);
    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&second_hash[..4]);
    checksum
}

fn sha256d(payload: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(payload);
    return hash.as_slice().to_vec();

}

pub fn construct_complete_message(command:Command,  payload: Vec<u8>) -> Vec<u8> {
    let mut complete_message = vec![];
    let magic_bytes: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
    let command : [u8; 12] = command.as_bytes();
    let payload_len = payload.len() as u32;
    let checksum = bitcoin_checksum(&payload);

    complete_message.extend(&magic_bytes);
    complete_message.extend(&command);
    complete_message.extend(&payload_len.to_le_bytes());
    complete_message.extend(&checksum);
    complete_message.extend(&payload);

    return complete_message;

}
