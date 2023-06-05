use colored::Colorize;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
pub struct Packet {
    pub direction: PacketDirection,
    pub protocol: Protocol,
    pub src_ip: Ipv4Addr,
    pub src_port: u16,
    pub dst_ip: Ipv4Addr,
    pub dst_port: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum PacketDirection {
    Send,
    Receive,
    Both,
}

#[derive(Debug, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    Unknown,
}

pub struct IpHeader {
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
}

impl Packet {
    pub fn new(
        direction: PacketDirection,
        protocol: Protocol,
        src_ip: Ipv4Addr,
        src_port: u16,
        dst_ip: Ipv4Addr,
        dst_port: u16,
        payload: Vec<u8>,
    ) -> Packet {
        Packet {
            direction,
            protocol,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            payload,
        }
    }

    pub fn from_ethernet(packet: &[u8]) -> Packet {
        let ethernet_header = Packet::parse_ethernet_header(packet);
        let ip_header = Packet::parse_ip_header(packet);
        let tcp_header = Packet::parse_tcp_header(packet);

        Packet {
            direction: Self::parse_direction(&packet),
            protocol: Self::parse_protocol(&packet),
            src_ip: Ipv4Addr::new(
                ip_header.src_ip[0],
                ip_header.src_ip[1],
                ip_header.src_ip[2],
                ip_header.src_ip[3],
            ),
            src_port: u16::from_be_bytes([tcp_header[0], tcp_header[1]]),
            dst_ip: Ipv4Addr::new(
                ip_header.dst_ip[0],
                ip_header.dst_ip[1],
                ip_header.dst_ip[2],
                ip_header.dst_ip[3],
            ),
            dst_port: u16::from_be_bytes([tcp_header[2], tcp_header[3]]),
            payload: Self::parse_payload(&packet),
        }
    }

    fn parse_udp_header(packet: &[u8]) -> [u8; 20] {
        let mut header = [0u8; 20];
        header.copy_from_slice(&packet[34..54]);
        header
    }

    fn parse_tcp_header(packet: &[u8]) -> [u8; 4] {
        let mut header = [0u8; 4];
        header.copy_from_slice(&packet[34..38]);
        header
    }

    fn parse_direction(packet: &[u8]) -> PacketDirection {
        if packet[0] == 0x00 {
            PacketDirection::Send
        } else if packet[0] == 0x01 {
            PacketDirection::Receive
        } else {
            PacketDirection::Both
        }
    }

    fn parse_protocol(packet: &[u8]) -> Protocol {
        if packet[0] == 0x00 {
            Protocol::TCP
        } else if packet[0] == 0x01 {
            Protocol::UDP
        } else {
            Protocol::Unknown
        }
    }

    fn parse_ethernet_header(packet: &[u8]) -> [u8; 14] {
        let mut header = [0u8; 14];
        header.copy_from_slice(&packet[0..14]);
        header
    }

    fn parse_ip_header(packet: &[u8]) -> IpHeader {
        IpHeader {
            src_ip: [packet[14], packet[15], packet[16], packet[17]],
            dst_ip: [packet[18], packet[19], packet[20], packet[21]],
        }
    }

    fn parse_payload(packet: &[u8]) -> Vec<u8> {
        packet[54..].to_vec()
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let direction = match self.direction {
            PacketDirection::Send => "Send",
            PacketDirection::Receive => "Receive",
            PacketDirection::Both => "Both",
        };

        let protocol = match self.protocol {
            Protocol::TCP => "TCP",
            Protocol::UDP => "UDP",
            Protocol::Unknown => "Unknown",
        };

        let direction = match self.direction {
            PacketDirection::Send => format!("{}", direction.green()),
            PacketDirection::Receive => format!("{}", direction.red()),
            PacketDirection::Both => format!("{}", direction.yellow()),
        };

        let protocol = match self.protocol {
            Protocol::TCP => format!("{}", protocol.green()),
            Protocol::UDP => format!("{}", protocol.red()),
            Protocol::Unknown => format!("{}", protocol.yellow()),
        };

        let direction = match self.direction {
            PacketDirection::Send => format!("{}", "-- SEND ->".green()),
            PacketDirection::Receive => format!("{}", "<- RECV --".red()),
            PacketDirection::Both => format!("{}", "<< BOTH >>".yellow()),
        };

        //pretty output with colors and in a table
        write!(
            f,
            "|  {:15}:{:5}  |  {:10}|  {:15}:{:5}  | Direction: {:<21} |  Protocol: {:18} |    {:5} bytes ",
            self.src_ip.to_string().green(),
            self.src_port.to_string().green(),
            "------->".to_string(),
            self.dst_ip.to_string().red(),
            self.dst_port.to_string().red(),
            direction,
            protocol,
            self.payload.len()
        )
    }
}
