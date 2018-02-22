
use std::{ptr, mem};

pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut table: [u8; 256] = unsafe { mem::uninitialized() };

    for i in 0..256 {
        table[i] = i as u8;
    }

    let mut output = Vec::with_capacity(data.len());

    for &byte in data {
        let idx = table.iter().position(|e| *e == byte).unwrap();
        for i in (1..idx + 1).rev() {
            table[i] = table[i - 1];
        }
        table[0] = byte;
        output.push(idx as u8);
    }

    output
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut table: [u8; 256] = unsafe { mem::uninitialized() };

    for i in 0..256 {
        table[i] = i as u8;
    }

    let mut output = Vec::with_capacity(data.len());

    for &idx in data {
        let idx = idx as usize;
        let byte = table[idx];
        for i in (1..idx + 1).rev() {
            table[i] = table[i - 1];
        }
        table[0] = byte;
        output.push(byte);
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
    }
}
