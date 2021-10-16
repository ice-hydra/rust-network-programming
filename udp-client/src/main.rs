use std::net::UdpSocket;
fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8081")?;
    socket.connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        let mut buf = [0u8; 512];
        let num = socket.recv(&mut buf)?;
        println!("recv: {}\n", std::str::from_utf8(&buf[..num]).unwrap());
    }
}
