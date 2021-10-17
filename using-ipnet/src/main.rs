use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use ipnet::{IpNet, Ipv4Net, Ipv6Net};
fn main() {
    let v4 = IpNet::V4(Ipv4Net::new(Ipv4Addr::new(10, 1, 1, 0), 24).unwrap());
    let v6 = IpNet::V6(Ipv6Net::new(Ipv6Addr::new(0xfd, 0, 0, 0, 0, 0, 0, 1), 24).unwrap());

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    let v4 = IpNet::V4(Ipv4Net::from_str("10.1.1.0/24").unwrap());
    let v6 = IpNet::V6(Ipv6Net::from_str("fd::1/24").unwrap());

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    let v4 = IpNet::V4("10.1.1.0/24".parse().unwrap());
    let v6 = IpNet::V6("fd::1/24".parse().unwrap());

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    let v4: Ipv4Net = "10.1.1.0/21".parse().unwrap();
    let v4 = IpNet::from(v4);
    let v6: Ipv6Net = "fd::1/24".parse().unwrap();
    let v6 = IpNet::from(v6);

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    let v4 = IpNet::from_str("10.1.1.0/24").unwrap();
    let v6 = IpNet::from_str("fd::1/24").unwrap();

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    let v4: IpNet = "10.1.1.0/24".parse().unwrap();
    let v6: IpNet = "fd::1/24".parse().unwrap();

    println!(
        "{} hostmask = {:?}, netmask = {:?}",
        v4,
        v4.hostmask(),
        v4.netmask()
    );
    println!(
        "{} hostmask = {:?}, netmask = {:?}",
        v6,
        v6.hostmask(),
        v6.netmask()
    );
}
