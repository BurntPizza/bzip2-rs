
use std::mem::*;
use std::iter::*;
use std::u32;

pub fn initial_encode(data: &[u8]) -> Vec<u8> {
    let n = data.len();
    if n == 0 { return vec![]; }
    assert!(n <= u32::MAX as usize);

    let mut output = Vec::with_capacity(1024);

    let mut i = 0;

    while i < n {
        let b = data[i];
        let mut run_len = 1;

        while run_len < 256 && i + run_len < n && data[i + run_len] == b {
            run_len += 1;
        }

        output.extend(repeat(b).take(run_len).take(4));

        if run_len >= 4 {
            debug_assert!(run_len <= 255);
            output.push(run_len as u8 - 4);
        }

        i += run_len;
    }

    output
}

pub fn initial_decode(encoded: &[u8]) -> Vec<u8> {
    type Chunk = (u8, u8, u8, u8);
    debug_assert_eq!(align_of::<Chunk>(), 1);

    let n = encoded.len();
    let mut output = Vec::with_capacity(1024);

    let mut i = 0;
    while i + size_of::<Chunk>() < n {
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

    output.reserve_exact(n - i);
    for i in i..n {
        output.push(encoded[i]);
    }

    output
}
