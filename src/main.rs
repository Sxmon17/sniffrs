use pnet::datalink; use pnet::datalink::Channel::Ethernet;

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
                dbg!(&packet);
                handle_packet(&packet);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(packet: &[u8]) {
    let ethernet = EthernetPacket::new(packet).unwrap();
    match ethernet.ethernet_type {
        EthernetType::Ipv4 => {
            ipv4_handler(&ethernet);
        }
        EthernetType::Ipv6 => {
            ipv6_handler(&ethernet);
        }
        _ => {
            println!("Unknown packet: {:?}", ethernet);
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    println!("IPv4 packet: {:?}", ethernet);
}

fn ipv6_handler(ethernet: &EthernetPacket) {
    println!("IPv6 packet: {:?}", ethernet);
}
