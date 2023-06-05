mod packet;

use pnet::datalink; use pnet::datalink::Channel::Ethernet;
use crate::packet::Packet;

fn main() {
    let interface = datalink::interfaces().into_iter().filter(|iface| iface.name == "enp4s0").next().expect("Could not find interface");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                Packet::from_ethernet(&packet);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}