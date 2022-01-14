use std::fs::File;
use std::io::{stdin, stdout, Cursor, Read, Write};
use std::str;

fn main() {
    // create debug log file
    let mut file = File::create("C:\\home\\Workspace\\WorkingFiles\\pearshim_debug.log").unwrap();

    // check for connectivity with peard
    // if pipe is good, send startup action
    send_action(0);

    loop {
        // get size of payload
        let mut sizebuffer = [0u8; 4];
        stdin().read(&mut sizebuffer).unwrap();
        let msg_size: u32 = u32::from_le_bytes(sizebuffer);

        // write payload size to log file
        file.write("reading bytes:".as_bytes()).unwrap();
        file.write(&sizebuffer).unwrap();
        file.write(&[':' as u8]).unwrap();

        let mut buffer = vec![0u8; msg_size as usize];
        stdin().read_exact(&mut buffer).unwrap();

        // write payload to log file
        file.write_all(&buffer).unwrap();
        file.write(&['\r' as u8, '\n' as u8]).unwrap();

        stdout().write_all(&sizebuffer).unwrap();
        stdout().write_all(&buffer).unwrap();
    }
}

fn send_action(action: u8) {
    let text = format!("{{\"action\":{},\"payload\":{{}}}}\r\n", action);
    send(text.as_str().as_bytes(), text.len());
}

fn send(buf: &[u8], size: usize) {
    let size = size as u32;
    stdout().write_all(&size.to_le_bytes()).unwrap();
    stdout().write_all(&buf).unwrap();
}

fn parse(buf: &[u8], size: usize) {
    if buf[0] == 123 {
        // buffer starts with '{'
        let main_obj = get_obj(buf).unwrap();
    } else if buf[0] == 34 {
        // buffer starts with '"'"
    }
}

fn get_obj(buf: &[u8]) -> Result<Vec<u8>, u8> {
    let mut index = 0;
    let mut defer = 0;
    for (i, val) in buf.iter().enumerate() {
        match val {
            123 => defer += 1, // '{'
            125 => match defer {
                0 => {
                    index = i;
                    break;
                }
                _ => defer -= 1,
            }, // '}'
            _ => {}
        }
    }
    if index > 0 {
        // successful
        return Ok(buf[1..index].to_vec());
    }

    Err(1)
}
