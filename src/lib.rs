use std::io;

pub mod uart;

pub trait SendRecv {
    fn send(&self, data: &[u8]) -> io::Result<usize>;

    fn recv(&self, data: &mut [u8]) -> io::Result<usize>;
}
