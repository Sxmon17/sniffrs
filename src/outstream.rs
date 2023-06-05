use crate::packet::{Packet, PacketDirection, Protocol};
use colored::Colorize;

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
            Protocol::ARP => "ARP",
            Protocol::ICMP => "ICMP",
            Protocol::DNS => "DNS",
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
            Protocol::ARP => format!("{}", protocol.blue()),
            Protocol::ICMP => format!("{}", protocol.magenta()),
            Protocol::DNS => format!("{}", protocol.cyan()),
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
