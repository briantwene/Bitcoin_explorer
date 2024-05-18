use std::{error::Error, net::IpAddr};

pub fn ip_string_to_bytes(ip_string: &str) -> Result<Vec<u8>, std::net::AddrParseError> {
    let ip_addr: IpAddr = ip_string.parse()?;
    let ip_bytes = match ip_addr {
        IpAddr::V4(ipv4_addr) => ipv4_addr.octets().to_vec(),
        IpAddr::V6(ipv6_addr) => ipv6_addr.octets().to_vec(),
    };
    Ok(ip_bytes)
}


pub fn read_var_int (payload:Vec<u8>) -> Result<(u64, usize), Box<dyn Error>>{
    let first_byte = payload[0];
    let (count, offset) = match first_byte {
        value if value < 0xFD => (value as u64, 1),
        0xFD => (u16::from_le_bytes(payload[1..3].try_into().unwrap()) as u64, 3),
        0xFE => (u32::from_le_bytes(payload[1..5].try_into().unwrap()) as u64, 5),
        0xFF => (u64::from_le_bytes(payload[1..9].try_into().unwrap()), 9),
        _ => return Err("Invalid count".into()),
    };

    Ok((count, offset))
}

pub fn read_bytes(data: &[u8], start: &mut usize, length: usize) -> Result<Vec<u8>, std::io::Error> {
    if *start + length > data.len() {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data"));
    }
    let result = data[*start..*start + length].to_vec();
    *start += length;
    Ok(result)
}

pub fn read_u32(data: &[u8], start: &mut usize) -> Result<u32, Box<dyn Error>> {
    Ok(u32::from_le_bytes(read_bytes(data, start, 4)?.try_into().unwrap()))
}

pub fn read_u64(data: &[u8], start: &mut usize) -> Result<u64, Box<dyn Error>> {
    Ok(u64::from_le_bytes(read_bytes(data, start, 8)?.try_into().unwrap()))
}

pub fn read_var_bytes(data: &[u8], start: &mut usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let (length, offset) = read_var_int(data[*start..].to_vec())?;
    *start += offset;
    Ok(read_bytes(data, start, length as usize)?)
}

pub const BITCOIN_MAGIC: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];