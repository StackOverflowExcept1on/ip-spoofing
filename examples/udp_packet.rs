use ip_spoofing::{etherparse::*, RawSocket, ReusablePacketWriter};

fn main() -> ip_spoofing::Result<()> {
    let socket = RawSocket::new()?;

    let mut writer = ReusablePacketWriter::new();

    socket.send_fake_udp_packet(
        &mut writer,
        [8, 8, 8, 8],
        1234,
        [127, 0, 0, 1],
        5678,
        b"hey",
        64,
    )?;

    Ok(())
}
