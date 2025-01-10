use std::io;
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd};
use nix::unistd::{read, write};
use std::fs::{File, OpenOptions};
use spidev::Spidev;

pub trait SendRecv {
    fn send(&mut self, data: &[u8]) -> io::Result<usize>;

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize>;
}

pub struct Uart {
    path: String,
    fd: File,
}

impl Uart {
    pub fn open(devpath: &str) -> io::Result<Uart> {
        let fd = OpenOptions::new().read(true).write(true).open(devpath)?;
        Ok(Uart {
            path: devpath.to_string(),
            fd,
        })
    }

    pub fn name(&self) -> &String {
        &self.path
    }
}
      
impl SendRecv for Uart {
    fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(write(self.fd.as_fd(), data)?)
    }

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize> {
        Ok(read(self.fd.as_raw_fd(), data)?)
    }
}

pub struct Spi {
    path: String,
    device: Spidev,
}

impl Spi {
    pub fn open(devpath: &str) -> io::Result<Spi> {
        let device = Spidev::open(devpath)?;
        Ok(Spi {
            path: devpath.to_string(),
            device,
        })
    }

    pub fn name(&self) -> &String {
        &self.path
    }
}
      
impl SendRecv for Spi {
    fn send(&mut self, data: &[u8]) -> io::Result<usize> {
        Ok(self.device.write(data)?)
    }

    fn recv(&mut self, data: &mut [u8]) -> io::Result<usize> {
        Ok(self.device.read(data)?)
    }
}
