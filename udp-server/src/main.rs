use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    loop {
        let mut buf = [0u8;512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("[READ] size: {}, src: {:?}", amt, src);

        let buf = &mut buf[..amt];
        buf.reverse();
        let sended_size =  socket.send_to(buf, src)?;
        println!("[SEND] size: {}, src: {:?} ", sended_size, src);
    }
}
