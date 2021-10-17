use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

fn main() {
    let mut v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);

    assert_eq!(v4.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(
        v6.ip(),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1))
    );

    assert_eq!(v4.port(), 8080);
    assert_eq!(v6.port(), 8080);

    assert_eq!("127.0.0.1:8080".parse(), Ok(v4));
    assert_eq!("[::ffff:0.0.0.1]:8080".parse(), Ok(v6));

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());

    v4.set_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)));

    println!("{:?}, {:?}", v4.to_string(), v6.to_string());
}
