use super::SendRecv;

use std::io;
use std::os::fd::{AsFd, AsRawFd};
use nix::unistd::{read, write};
use std::fs::{File, OpenOptions};

pub struct Uart {
    path: String,
    fd: File,
}

impl Uart {
    pub fn open(devpath: &String) -> io::Result<Uart> {
        let fd = OpenOptions::new().read(true).write(true).open(devpath)?;
        Ok(Uart {
            path: devpath.clone(),
            fd,
        })
    }

    pub fn name(&self) -> &String {
        &self.path
    }
}
      
impl SendRecv for Uart {
    fn send(&self, data: &[u8]) -> io::Result<usize> {
        Ok(write(self.fd.as_fd(), data)?)
    }

    fn recv(&self, data: &mut [u8]) -> io::Result<usize> {
        Ok(read(self.fd.as_raw_fd(), data)?)
    }
}
