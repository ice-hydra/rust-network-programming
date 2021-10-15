use std::net::{TcpListener, TcpStream};
use std::{
    io::{self, Read, Write},
    thread, time,
};

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut threads = vec![];

    for stream in listener.incoming() {
        let mut stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(&mut stream).unwrap_or_else(|err| eprintln!("{:?}", err));
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }

    Ok(())
}
