
use std::{ptr, mem};

pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut table: [u8; 256] = unsafe { mem::uninitialized() };

    for i in 0..256 {
        table[i] = i as u8;
    }

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

        unsafe {
            let src = table.as_ptr();
            let dst = table.as_mut_ptr().offset(1);
            ptr::copy(src, dst, idx);
        }
        table[0] = byte;
    }

    output
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut table: [u8; 256] = unsafe { mem::uninitialized() };

    for i in 0..256 {
        table[i] = i as u8;
    }

    let mut output = Vec::with_capacity(data.len());
    unsafe { output.set_len(data.len()); }

    for (idx, ptr) in data.iter().cloned().zip(output.iter_mut()) {
        let idx = idx as usize;
        let byte = table[idx];
        *ptr = byte;

        if idx == 0 { continue; } // fast path

        unsafe {
            let src = table.as_ptr();
            let dst = table.as_mut_ptr().offset(1);
            ptr::copy(src, dst, idx);
        }
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
    }
}
