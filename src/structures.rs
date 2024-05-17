// version payload


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



