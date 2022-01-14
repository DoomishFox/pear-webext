//use std::fs::File;
use std::cmp;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    // create debug log file
    //let mut file = File::create("C:\\home\\Workspace\\WorkingFiles\\pearshim_debug.log").unwrap();

    // check for connectivity with peard
    // if pipe is good, send startup action
    send_action(0);

    loop {
        // get size of payload
        let mut sizebuffer = [0u8; 4];
        stdin().read(&mut sizebuffer).unwrap();
        let size = u32::from_le_bytes(sizebuffer) as usize;

        let mut buffer = vec![0u8; size];
        stdin().read_exact(&mut buffer).unwrap();
        let packet = parse(&buffer);
        eprintln!("{:?}", packet);
    }
}

fn send_action(action: u8) {
    let text = format!("{{\"action\":{},\"payload\":{{}}}}\r\n", action);
    eprintln!("assembling message: {}", text);
    send(text.as_str().as_bytes(), text.len());
}

fn send(buf: &[u8], size: usize) {
    let size = size as u32;
    eprintln!("sending as: {:?}", buf);
    stdout().write_all(&size.to_le_bytes()).unwrap();
    stdout().write_all(&buf).unwrap();
}

fn parse(buf: &[u8]) {
    eprintln!("received buffer: {:?}", buf);
    if buf[0] == 123 {
        // buffer starts with '{'
        let main_obj = get_obj(&buf).unwrap();
        eprintln!("parsed root obj: {:?}", main_obj);

        // make sure object contains an action, otherwise it isnt a valid payload
        if compare(&main_obj[0..8], &[34, 97, 99, 116, 105, 111, 110, 34]) == cmp::Ordering::Equal {
            // get action type
            let action = main_obj[9];

            match action {
                1 => { // request devices
                }
                2 => { // send to device
                }
                other => {
                    eprintln!("payload action '{}' not recognized", other);
                }
            }
        }
    } else if buf[0] == 34 {
        // buffer starts with '"'"
    }
}

fn get_obj(buf: &[u8]) -> Result<Vec<u8>, &str> {
    let mut index = 0;
    let mut defer = 0;
    for (i, val) in buf.iter().skip(1).enumerate() {
        match val {
            123 => defer += 1, // '{'
            125 => match defer {
                0 => {
                    index = i;
                    eprintln!("found end of object at index: {}", index);
                    break;
                }
                _ => defer -= 1,
            }, // '}'
            _ => {}
        }
    }
    if index > 0 {
        // successful
        return Ok(buf[1..index + 1].to_vec());
    }

    Err("invalid parse, failed to get JSON object")
}

fn count_until(buf: &[u8], sep: u8) -> usize {
    for (i, a) in buf.iter().enumerate() {
        match a.cmp(&sep) {
            cmp::Ordering::Equal => return i,
            _ => continue,
        }
    }
    0
}

fn compare(a: &[u8], b: &[u8]) -> cmp::Ordering {
    for (ai, bi) in a.iter().zip(b.iter()) {
        match ai.cmp(&bi) {
            cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }

    /* if every single element was equal, compare length */
    a.len().cmp(&b.len())
}
