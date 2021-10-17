use pnet::{
    datalink::{self, Channel::Ethernet, NetworkInterface},
    packet::{
        ethernet::{EtherTypes, EthernetPacket},
        ip::IpNextHeaderProtocols,
        ipv4::Ipv4Packet,
        tcp::TcpPacket,
        Packet,
    },
};

use std::env;
fn main() {
    let interface_name = env::args().nth(1).unwrap();
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|i: &NetworkInterface| i.name.eq(&interface_name))
        .next()
        .expect("Error get interface");
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Other"),
        Err(e) => panic!("{:?}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_pack(&packet);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn handle_pack(eth: &EthernetPacket) {
    match eth.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(eth.payload());
            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());
                        if let Some(tcp) = tcp {
                            println!(
                                "Tcp packet {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    }
                    _ => println!("ignore"),
                }
            }
        }
        _ => println!("ignore"),
    }
}
