use super::Result;

use etherparse::*;
use rand::Rng;

/// Wrapper around `Vec<u8>` that is pre-allocates memory for writing packet bytes
#[derive(Debug)]
pub struct ReusablePacketWriter {
    inner: Vec<u8>,
}

impl ReusablePacketWriter {
    /// Creates wrapper around `Vec<u8>` with pre-allocated `u16::MAX` (max possible MTU size)
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(u16::MAX as _),
        }
    }

    /// Takes slice of memory from this wrapper
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice()
    }

    /// Builds IPv4 header with randomized `identification` field
    #[inline]
    pub fn build_ipv4_header(
        time_to_live: u8,
        protocol: IpNumber,
        source: [u8; 4],
        destination: [u8; 4],
    ) -> IpHeaders {
        let identification = rand::rng().random();
        IpHeaders::Ipv4(
            Ipv4Header {
                identification,
                time_to_live,
                protocol,
                source,
                destination,
                ..Default::default()
            },
            Default::default(),
        )
    }

    /// Writes UDP packet into buffer with given parameters
    #[inline]
    pub fn write_udp_packet(
        &mut self,
        source: [u8; 4],
        source_port: u16,
        destination: [u8; 4],
        destination_port: u16,
        payload: &[u8],
        time_to_live: u8,
    ) -> Result<()> {
        self.inner.clear();

        let ip_header = Self::build_ipv4_header(time_to_live, IpNumber::UDP, source, destination);

        PacketBuilder::ip(ip_header)
            .udp(source_port, destination_port)
            .write(&mut self.inner, payload)?;

        Ok(())
    }

    /// Writes TCP-SYN packet into buffer with given parameters
    #[inline]
    pub fn write_tcp_syn_packet(
        &mut self,
        source: [u8; 4],
        source_port: u16,
        destination: [u8; 4],
        destination_port: u16,
        payload: &[u8],
        time_to_live: u8,
        sequence_number: u32,
        window_size: u16,
    ) -> Result<()> {
        self.inner.clear();

        let ip_header = Self::build_ipv4_header(time_to_live, IpNumber::TCP, source, destination);

        PacketBuilder::ip(ip_header)
            .tcp(source_port, destination_port, sequence_number, window_size)
            .syn()
            .write(&mut self.inner, payload)?;

        Ok(())
    }
}
