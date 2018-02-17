
#[cfg(test)]
extern crate bzip2;
#[cfg(test)]
#[macro_use]
extern crate proptest;

pub fn bwt(data: &[u8]) -> (Vec<u8>, u32) {
    let n = data.len();
    if n == 0 { return (vec![], 0); }

    let mut matrix: Vec<u32> = (0..n as u32).collect();
    matrix.sort_unstable_by(|a, b| {
        use std::cmp::Ordering;

        let mut a = *a as usize;
        let mut b = *b as usize;

        for _ in 0..n {
            if a >= n { a = 0; }
            if b >= n { b = 0; }
            match data[a].cmp(&data[b]) {
                Ordering::Equal => {}
                non_eq => return non_eq,
            }
            a += 1;
            b += 1;
        }

        Ordering::Equal
    });

    // let idx = matrix.iter().position(|row| *row == 0).unwrap();
    let mut idx = 0;
    let last_col: Vec<u8> = matrix.into_iter().enumerate().map(|(i, row)| {
        if row == 0 {
            idx = i as u32;
        }
        data[(row as usize + n - 1) % n]
    }).collect();

    (last_col, idx)
}

pub fn ibwt(data: &[u8], start: u32) -> Vec<u8> {
    let n = data.len();
    if n == 0 { return vec![]; }
    assert!(n <= std::u32::MAX as usize);

    let mut shortcut = Vec::with_capacity(n);
    unsafe { shortcut.set_len(n); }
    let mut counts = [0u32; 256];

    for i in 0..n {
        let current_byte = data[i] as usize;
        shortcut[i] = counts[current_byte];
        counts[current_byte] += 1;
    }

    let mut first_occ = [std::u32::MAX; 256];
    let total_distinct = counts.iter().filter(|n| **n > 0).count();
    let mut num_distinct = 0;
    let first_col = sorted(data, &counts);

    for i in 0..n {
        let idx = first_col[i] as usize;
        if first_occ[idx] == std::u32::MAX {
            first_occ[idx] = i as _;
            num_distinct += 1;
            if num_distinct >= total_distinct {
                break;
            }
        }
    }

    let mut output = first_col; // reuse memory
    let mut local_idx = start as usize;

    for i in 0..n {
        let next_byte = data[local_idx];
        output[n - i - 1] = next_byte;
        local_idx = first_occ[next_byte as usize] as usize + shortcut[local_idx] as usize;
    }

    output
}

pub fn sorted(data: &[u8], counts: &[u32; 256]) -> Vec<u8> {
    unsafe {
        let mut output = Vec::with_capacity(data.len());
        output.set_len(data.len());
        let mut ptr: *mut u8 = output.as_mut_ptr();

        for i in 0..256 {
            let count = counts[i];
            std::ptr::write_bytes(ptr, i as u8, count as usize);
            ptr = ptr.offset(count as isize);
        }

        output
    }
}

#[cfg(any(test, feature = "bench"))]
pub fn bwt_ref(data: &[u8]) -> (Vec<u8>, u32) {
    use std::collections::VecDeque;

    if data.is_empty() {
        return (vec![], 0);
    }

    let n = data.len();
    let mut matrix = Vec::with_capacity(n);

    let mut row: VecDeque<u8> = data.iter().cloned().collect();;
    for _ in 0..n {
        let b = row.pop_back().unwrap();
        row.push_front(b);
        matrix.push(row.clone());
    }
    debug_assert_eq!(matrix.len(), n);
    matrix.sort();

    let idx = matrix.iter().position(|row| &*row.iter().cloned().collect::<Vec<_>>() == data).unwrap();
    let last_col = matrix.into_iter().map(|row| row[row.len() - 1]).collect();

    (last_col, idx as u32)
}

#[cfg(any(test, feature = "bench"))]
pub fn ibwt_ref(data: &[u8], start: u32) -> Vec<u8> {
    use std::collections::VecDeque;

    if data.is_empty() {
        return vec![];
    }

    let n = data.len();

    let mut matrix = (0..n).map(|_| VecDeque::with_capacity(n)).collect::<Vec<_>>();

    for _ in 0..n {
        for (row, ch) in matrix.iter_mut().zip(&data[..]) {
            row.push_front(*ch);
        }
        matrix.sort();
    }

    std::mem::replace(&mut matrix[start as usize], Default::default()).into()
}

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use bzip2::Compression;
    use bzip2::read::*;

    use proptest::string::*;

    use *;

    proptest! {
        #[test]
        fn bwt_reference_round_trip(ref data in bytes_regex(".*").unwrap()) {
            let (bwt, idx) = bwt_ref(&data[..]);
            let ibwt = ibwt_ref(&bwt[..], idx);
            prop_assert_eq!(&ibwt[..], &data[..])
        }

        #[test]
        fn bwt_round_trip(ref data in bytes_regex(".*").unwrap()) {
            let (bwt, idx) = bwt(&data[..]);
            let out = ibwt(&bwt[..], idx);
            prop_assert_eq!(&out[..], &data[..])
        }

        #[test]
        fn test_bwt(ref data in bytes_regex(".*").unwrap()) {
            let (bwt, idx) = bwt(&data[..]);
            let ibwt = ibwt_ref(&bwt[..], idx);
            prop_assert_eq!(&ibwt[..], &data[..])
        }

        #[test]
        fn test_ibwt(ref data in bytes_regex(".*").unwrap()) {
            let (bwt, idx) = bwt_ref(&data[..]);
            let ibwt = ibwt(&bwt[..], idx);
            prop_assert_eq!(&ibwt[..], &data[..]);
        }

        #[test]
        fn counting_sort(ref data in bytes_regex(".*").unwrap()) {
            let mut counts = [0; 256];
            for i in 0..data.len() {
                counts[data[i] as usize] += 1;
            }
            let c_sorted = sorted(&data[..], &counts);
            let mut std_sorted = data.to_owned();
            std_sorted.sort_unstable();
            prop_assert_eq!(c_sorted, std_sorted);
        }
    }

    // #[test]
    // fn it_works() {
    //     let data = "Hello World!".as_bytes();
    //     let mut encoder = BzEncoder::new(data, Compression::Best);
    //     let mut compressed = vec![];
    //     encoder.read_to_end(&mut compressed).unwrap();
    //     println!("raw: {:?}", compressed);
    //     println!("hex: {}", compressed.iter().map(|b| format!("{:X}", b)).collect::<String>());
    //     println!("ascii: {}", compressed.iter().map(|b| format!("{}", (*b as char).escape_default())).collect::<String>());
    // }
}
