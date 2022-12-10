use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut f = File::open("inputs/day6.txt").unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    let chars_processed = start_of_transmission(&s, 4);
    if let Some(chars) = chars_processed {
        println!("Processed {} chars", chars);
    }

    let chars_processed = start_of_transmission(&s, 14);
    if let Some(chars) = chars_processed {
        println!("Processed {} chars", chars);
    }
}

fn start_of_transmission(code: &str, preamble_length: usize) -> Option<usize> {
    let mut buf = Vec::with_capacity(preamble_length);

    for (i, b) in code.bytes().enumerate() {
        if buf.len() < preamble_length {
            buf.push(b);
        } else {
            buf[i % preamble_length] = b;
            if buf.iter().copied().collect::<HashSet<u8>>().len() == preamble_length {
                return Some(i + 1);
            }
        }
    }
    None
}
