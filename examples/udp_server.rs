use std::net::UdpSocket;
use std::{io, str};

/// This example creates UDP server on `"0.0.0.0:5678` to listen for packets
fn main() -> io::Result<()> {
    let addr = "0.0.0.0:5678";
    let socket = UdpSocket::bind(addr)?;

    println!("Listening for UDP packets on {addr}...");

    let mut buf = [0; u16::MAX as usize];
    loop {
        let (len, addr) = socket.recv_from(&mut buf)?;

        let bytes = &buf[..len];
        let str = str::from_utf8(bytes).ok();

        println!("got {len} bytes {bytes:02X?} {str:?} from {addr}")
    }
}
