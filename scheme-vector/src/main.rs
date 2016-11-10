extern crate syscall;

use syscall::scheme::SchemeMut;
use syscall::error::{Error, Result, ENOENT, EBADF, EINVAL};
use std::cmp::min;

struct VecScheme {
    vec: Vec<u8>,
}

impl VecScheme {
    fn new() -> VecScheme {
        VecScheme {
            vec: Vec::new(),
        }
    }
}

impl SchemeMut for VecScheme {
    fn open(&mut self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Result<usize> {
        self.vec.extend_from_slice(path);
        Ok(path.len())
    }

    fn read(&mut self, id: usize, buf: &mut [u8]) -> Result<usize> {
        let res = min(buf.len(), self.vec.len());

        for b in buf {
            *b = if let Some(x) = self.vec.pop() {
                x
            } else {
                break;
            }
        }

        Ok(res)
    }

    fn write(&mut self, id: usize, buf: &[u8]) -> Result<usize> {
        for &i in buf {
            self.vec.push(i);
        }

        Ok(buf.len())
    }
}

fn main() {
    use syscall::data::Packet;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::mem::size_of;

    let mut scheme = VecScheme::new();
    // Create the handler
    let mut socket = File::create(":vec").unwrap();
    loop {
        let mut packet = Packet::default();
        println!("{:?}", packet);
        while socket.read(&mut packet).unwrap() == size_of::<Packet>() {
            scheme.handle(&mut packet);
            socket.write(&packet).unwrap();
        }
    }
}
