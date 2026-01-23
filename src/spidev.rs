use std::io;

pub struct Spidev {}

impl Spidev {
    pub fn open(_devpath: &str) -> io::Result<Self> {
        Ok(Self {})
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        Ok(data.len())
    }

    pub fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        Ok(data.len())
    }
}
