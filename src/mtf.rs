
use std::{ptr, mem};

pub struct Encoder<I> {
    table: [u8; 256],
    src: I,
}

pub struct Decoder<I> {
    table: [u8; 256],
    src: I,
}

impl<I> Encoder<I> {
    pub fn new(src: I) -> Self {
        Encoder {
            table: table(),
            src,
        }
    }
}

impl<I> Decoder<I> {
    pub fn new(src: I) -> Self {
        Decoder {
            table: table(),
            src,
        }
    }
}

impl<I> Iterator for Encoder<I> where I: Iterator<Item = u8> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.src.next() {
            Some(byte) => {
                // fast path
                if self.table[0] == byte {
                    return Some(0);
                }

                let idx = self.table.iter().position(|e| *e == byte).unwrap_or(0);
                shift_table(&mut self.table, idx);
                self.table[0] = byte;

                Some(idx as u8)
            }
            _ => None
        }
    }
}

impl<I> Iterator for Decoder<I> where I: Iterator<Item = u8> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        match self.src.next() {
            Some(idx) => {
                let idx = idx as usize;
                let byte = self.table[idx];

                if idx != 0 {
                    // slow path
                    shift_table(&mut self.table, idx);
                    self.table[0] = byte;
                }

                Some(byte)
            }
            _ => None
        }
    }
}


// moves a 'hole' at `idx` to index 0 by shifting [0..idx - 1] to [1..idx]
#[inline(always)]
fn shift_table(table: &mut [u8; 256], idx: usize) {
    unsafe {
        let src = table.as_ptr();
        let dst = table.as_mut_ptr().offset(1);
        ptr::copy(src, dst, idx);
    }
}

fn table() -> [u8; 256] {
    let mut table: [u8; 256] = unsafe { mem::uninitialized() };
    for i in 0..256 {
        table[i] = i as u8;
    }
    table
}

pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut table = table();
    let mut output  = Vec::with_capacity(data.len());
    unsafe { output.set_len(data.len()); }

    for (byte, ptr) in data.iter().cloned().zip(output.iter_mut()) {
        // fast path
        if table[0] == byte {
            *ptr = 0;
            continue;
        }

        let idx = (&table[1..]).iter().position(|e| *e == byte).map(|i| i + 1).unwrap_or(0);
        *ptr = idx as u8;

        shift_table(&mut table, idx);
        table[0] = byte;
    }

    output
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut table = table();
    let mut output = Vec::with_capacity(data.len());
    unsafe { output.set_len(data.len()); }

    for (idx, ptr) in data.iter().cloned().zip(output.iter_mut()) {
        let idx = idx as usize;
        let byte = table[idx];
        *ptr = byte;

        if idx == 0 { continue; } // fast path

        shift_table(&mut table, idx);
        table[0] = byte;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::string::*;

    #[test]
    fn test_1() {
        let case = b"bananaaa";
        let expected = vec![b'b', b'a' + 1, b'n', 1, 1, 1, 0, 0];
        
        assert_eq!(encode(case), expected);
    }

    proptest! {
        #[test]
        fn test_roundtrip(ref data in bytes_regex(".*").unwrap()) {
            let encoded = encode(data);
            let decoded = decode(&encoded);
            prop_assert_eq!(&decoded, data);
        }

        #[test]
        fn test_encode_iter(ref data in bytes_regex(".*").unwrap()) {
            prop_assert_eq!(Encoder::new(data.iter().cloned()).collect::<Vec<_>>(), encode(data));
        }

        #[test]
        fn test_decode_iter(ref data in bytes_regex(".*").unwrap()) {
            let data = encode(data);
            prop_assert_eq!(Decoder::new(data.iter().cloned()).collect::<Vec<_>>(), decode(&data));
        }
    }
}
