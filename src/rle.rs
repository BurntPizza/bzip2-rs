
use std::mem::*;
use std::iter::*;

pub struct Encoder {
    block: Vec<u8>,
    max_size: usize,
    in_len: usize,
    in_char: u16,
}

impl Encoder {
    pub fn new(block_size: usize) -> Self {
        Encoder {
            block: Vec::with_capacity(block_size),
            max_size: block_size,
            in_len: 0,
            in_char: 256,
        }
    }

    pub fn encode(&mut self, data: &[u8]) -> usize {
        let mut i = 0;

        while i < data.len() {
            if self.block.len() >= self.max_size {
                break;
            }

            let b = data[i] as u16;

            if b != self.in_char && self.in_len == 1 {
                let ib = self.in_char as u8;
                // update crc ib
                self.block.push(ib);
                self.in_char = b;
            } else if b != self.in_char || self.in_len == 255 {
                if self.in_char < 256 {
                    self.add_pair();
                }
                self.in_char = b;
                self.in_len = 1;
            } else {
                self.in_len += 1;
            }
            i += 1;
        }

        i
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.flush();
        self.block
    }

    fn is_empty(&self) -> bool {
        self.in_char > 255 || self.in_len == 0
    }

    fn flush(&mut self) {
        if self.in_char < 256 {
            self.add_pair();
        }
    }

    fn add_pair(&mut self) {
        debug_assert!(self.in_len > 0);

        let ch = self.in_char as u8;

        for _ in 0..self.in_len {
            // update crc ch
        }

        self.block.extend(repeat(ch).take(self.in_len).take(4));
        if self.in_len >= 4 {
            self.block.push(self.in_len as u8 - 4);
        }
    }
}

pub fn initial_decode(encoded: &[u8]) -> Vec<u8> {
    type Chunk = (u8, u8, u8, u8);
    debug_assert_eq!(align_of::<Chunk>(), 1);

    let n = encoded.len();
    let s = size_of::<Chunk>();
    let mut output = Vec::with_capacity(1024);

    let mut i = 0;
    while i + s < n - 1 {
        let (a, b, c, d) = unsafe { *(encoded.as_ptr().offset(i as isize) as *const Chunk) };
        let mut run_len = 1;
        if a == b {
            run_len += 1;
            if a == c {
                run_len += 1;
                if a == d {
                    i += 1 + run_len;
                    run_len += 1 + encoded[i] as usize;
                    output.extend(repeat(a).take(run_len));
                    continue;
                }
            }
        }

        i += run_len;
        output.extend(repeat(a).take(run_len));
    }
    println!("debug: {:?}", &output[..]);
    println!("encoded: {:?}", &encoded[i..]);

    if i < n && output.len() >= 4 {
        let on = output.len();
        let b = *output.last().unwrap();
        let b = [b; 4];
        if &output[on - 4..] == &b[..] && encoded[i] == 0 {
            // skip zero
            i += 1;
        }
    }

    output.reserve_exact(n - i);
    for i in i..n {
        output.push(encoded[i]);
    }

    output
}
