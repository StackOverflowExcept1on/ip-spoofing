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
        protocol: u8,
        source: [u8; 4],
        destination: [u8; 4],
    ) -> IpHeader {
        let mut ipv4_header = Ipv4Header::default();
        ipv4_header.identification = rand::thread_rng().gen();
        ipv4_header.time_to_live = time_to_live;
        ipv4_header.protocol = protocol;
        ipv4_header.source = source;
        ipv4_header.destination = destination;

        IpHeader::Version4(ipv4_header, Default::default())
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

        let ip_header = Self::build_ipv4_header(time_to_live, ip_number::UDP, source, destination);

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

        let ip_header = Self::build_ipv4_header(time_to_live, ip_number::TCP, source, destination);

        PacketBuilder::ip(ip_header)
            .tcp(source_port, destination_port, sequence_number, window_size)
            .syn()
            .write(&mut self.inner, payload)?;

        Ok(())
    }
}
