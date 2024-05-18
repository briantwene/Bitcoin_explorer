// version payload

use sha2::{Digest, Sha256};


#[derive(Debug)]
pub struct BitcoinMessage {
    pub magic: u32,
    pub command: String,
    pub length: u32,
    pub payload: Vec<u8>
}

pub struct VersionPayload {
    pub version: i32,
    pub services: u64,
    pub timestamp: i64,
    pub addr_recv: NetAddr,
    pub addr_from: NetAddr,
    pub nonce: u64,
    pub user_agent: Vec<u8>,
    pub start_height: i32,
    pub relay: bool,
}

pub struct NetAddr {
    // pub time: u32,
    pub services: u64,
    pub ip_v6_4: String,
    pub port: u16
}

#[derive(Clone)]
pub struct BlockData {
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub block_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
}


impl BlockData {
    fn calculate_hash(&mut self) {
        let mut headers = Vec::new();
        headers.extend(&self.version.to_le_bytes());
        headers.extend(&self.prev_block_hash);
        headers.extend(&self.merkle_root);
        headers.extend(&self.timestamp.to_le_bytes());
        headers.extend(&self.bits.to_le_bytes());
        headers.extend(&self.nonce.to_le_bytes());

        let first_hash = Sha256::digest(&headers);
        let second_hash = Sha256::digest(&first_hash);
        self.block_hash = second_hash.to_vec();
    }

    pub fn convert_date(&self) -> String {
        let date = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        date.format("%A %e %B %Y at %H:%M").to_string()
    }
}


pub enum Command {
    Version,
    Verack,
    Pong,
    GetData
}


impl Command {
    pub fn as_bytes(&self) -> [u8; 12] {
        match self {
            Command::Version => *b"version\0\0\0\0\0",
            Command::Verack => *b"verack\0\0\0\0\0\0",
            Command::Pong => *b"pong\0\0\0\0\0\0\0\0",
            Command::GetData => *b"getdata\0\0\0\0\0",
        }
    }
}




#[derive(Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub locktime: u32,
}

#[derive(Clone)]
pub struct TransactionInput {
    pub prev_tx_hash: [u8; 32],
    pub prev_output_index: u32,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Clone)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pub_key: Vec<u8>,
}