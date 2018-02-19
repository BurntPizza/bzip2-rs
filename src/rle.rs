
use std::mem::*;
use std::iter::*;

// fill buffer up to capacity, returning i, the amount of data was consumed (&data[i..] is unconsumed)
// TODO?: handle adding new data into a partially-filled buffer correctly?
pub fn initial_encode(data: &[u8], buffer: &mut Vec<u8>) -> usize {
    let limit = buffer.capacity() - buffer.len();
    let n = data.len();
    let mut added = 0;
    let mut i = 0;

    while i < n && added < limit {
        let remaining = limit - added;

        if remaining >= 5 {
            // encode up to full run
            let b = data[i];
            let mut run_len = 1;
            let pre_len = buffer.len();

            while run_len < 256 && i + run_len < n && data[i + run_len] == b {
                run_len += 1;
            }

            buffer.extend(repeat(b).take(run_len).take(4));

            if run_len >= 4 {
                debug_assert!(run_len <= 255);
                buffer.push(run_len as u8 - 4);
            }

            added += buffer.len() - pre_len;
            i += run_len;
        } else {
            let run_len = remaining.min(n - i);
            buffer.extend(&data[i..i + run_len]);
            i += run_len;
            break;
        }

    }

    return i;
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
