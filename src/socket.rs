use super::Result;

use crate::ReusablePacketWriter;
use nix::sys::socket::{
    sendto, socket, AddressFamily, MsgFlags, SockFlag, SockProtocol, SockType, SockaddrIn,
};
use nix::unistd::close;
use std::os::fd::{AsRawFd, RawFd};

#[derive(Debug)]
pub struct RawSocket {
    fd: RawFd,
}

impl RawSocket {
    #[inline]
    pub fn new() -> Result<Self> {
        let fd = socket(
            AddressFamily::Inet,
            SockType::Raw,
            SockFlag::empty(),
            Some(SockProtocol::Raw),
        )?;
        Ok(Self { fd })
    }

    #[inline]
    pub fn sendto(&self, buf: &[u8], addr: [u8; 4]) -> Result<usize> {
        let [a, b, c, d] = addr;
        let addr = SockaddrIn::new(a, b, c, d, 0);
        let len = sendto(self.fd, buf, &addr, MsgFlags::empty())?;
        Ok(len)
    }

    #[inline]
    pub fn send_fake_udp_packet(
        &self,
        writer: &mut ReusablePacketWriter,
        source: [u8; 4],
        source_port: u16,
        destination: [u8; 4],
        destination_port: u16,
        payload: &[u8],
        time_to_live: u8,
    ) -> Result<usize> {
        writer.write_udp_packet(
            source,
            source_port,
            destination,
            destination_port,
            payload,
            time_to_live,
        )?;
        self.sendto(writer.as_slice(), destination)
    }

    #[inline]
    pub fn send_fake_tcp_syn_packet(
        &self,
        writer: &mut ReusablePacketWriter,
        source: [u8; 4],
        source_port: u16,
        destination: [u8; 4],
        destination_port: u16,
        payload: &[u8],
        time_to_live: u8,
        sequence_number: u32,
        window_size: u16,
    ) -> Result<usize> {
        writer.write_tcp_syn_packet(
            source,
            source_port,
            destination,
            destination_port,
            payload,
            time_to_live,
            sequence_number,
            window_size,
        )?;
        self.sendto(writer.as_slice(), destination)
    }
}

impl AsRawFd for RawSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for RawSocket {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}
