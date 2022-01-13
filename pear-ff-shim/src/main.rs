use std::io::{stdin, Read, stdout, Write};
use std::fs::File;

fn main() {
    let mut file = File::create("C:\\home\\Workspace\\WorkingFiles\\pear_debug.txt").unwrap();
    loop {
        // get size of payload
        let mut sizebuffer = [0u8; 4];
        stdin().read(&mut sizebuffer).unwrap();
        let msg_size: u32 = u32::from_le_bytes(sizebuffer);
        
        file.write("reading bytes:".as_bytes()).unwrap();
        file.write(&sizebuffer).unwrap();
        file.write(&[':' as u8]).unwrap();

        let mut buffer = vec![0u8; msg_size as usize];
        stdin().read_exact(&mut buffer).unwrap();

        file.write_all(&buffer).unwrap();
        file.write(&['\r' as u8, '\n' as u8]).unwrap();

        stdout().write_all(&sizebuffer).unwrap();
        stdout().write_all(&buffer).unwrap();
    }
}
