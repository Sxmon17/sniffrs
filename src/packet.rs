use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub direction: PacketDirection,
    pub protocol: Protocol,
    pub src_ip: Ipv4Addr,
    pub src_port: u16,
    pub dst_ip: Ipv4Addr,
    pub dst_port: u16,
    pub payload: Vec<u8>
}

#[derive(Debug, PartialEq)]
pub enum PacketDirection {
    Send,
    Receive,
    Both
}

#[derive(Debug, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    Unknown
}

pub struct IpHeader {
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
}

impl Packet {
    pub fn new(direction: PacketDirection, protocol: Protocol, src_ip: Ipv4Addr, src_port: u16, dst_ip: Ipv4Addr, dst_port: u16, payload: Vec<u8>) -> Packet {
        Packet {
            direction,
            protocol,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            payload
        }
    }
}