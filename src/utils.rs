use std::net::IpAddr;

pub fn ip_string_to_bytes(ip_string: &str) -> Result<Vec<u8>, std::net::AddrParseError> {
    let ip_addr: IpAddr = ip_string.parse()?;
    let ip_bytes = match ip_addr {
        IpAddr::V4(ipv4_addr) => ipv4_addr.octets().to_vec(),
        IpAddr::V6(ipv6_addr) => ipv6_addr.octets().to_vec(),
    };
    Ok(ip_bytes)
}