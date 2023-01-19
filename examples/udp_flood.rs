use ip_spoofing::{self, rand::*, RawSocket, ReusablePacketWriter};

fn main() -> ip_spoofing::Result<()> {
    let socket = RawSocket::new()?;

    let mut writer = ReusablePacketWriter::new();
    let mut rng = thread_rng();

    loop {
        let ret = socket.send_fake_udp_packet(
            &mut writer,
            rng.gen(),
            rng.gen(),
            [127, 0, 0, 1],
            5678,
            b"hey",
            64,
        );

        if let Err(err) = ret {
            println!("{err:?}");
        }
    }
}
