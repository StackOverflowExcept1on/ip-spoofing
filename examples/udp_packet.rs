use ip_spoofing::{self, RawSocket, ReusablePacketWriter};

/// This example shows how to generate fake UDP packet
/// that delivers `b"hey"` bytes from "8.8.8.8:1234" to "127.0.0.1:5678".
///
/// I.e. the attacker changes its IPv4 address to 8.8.8.8 (Google Public DNS)
fn main() -> ip_spoofing::Result<()> {
    //wrapper around raw sockets, requires root privileges
    let socket = RawSocket::new()?;

    //wrapper for writing packets in pre-allocated memory
    let mut writer = ReusablePacketWriter::new();

    //sends fake UDP packet
    socket.send_fake_udp_packet(
        &mut writer,
        [8, 8, 8, 8],   //source IPv4 address
        1234,           //source port
        [127, 0, 0, 1], //destination IPv4 address
        5678,           //destination port
        b"hey",         //data
        64,             //TTL on most Linux machines is 64
    )?;

    Ok(())
}
