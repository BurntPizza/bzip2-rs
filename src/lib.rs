
#[cfg(test)]
extern crate bzip2;
#[cfg(test)]
#[macro_use]
extern crate proptest;

use std::cmp::Ordering;
use std::cmp::Ordering::*;

pub mod rle;


pub fn bwt(data: &[u8]) -> (Vec<u8>, u32) {
    let n = data.len();
    if n == 0 { return (vec![], 0); }
    assert!(n <= std::u32::MAX as usize);

    let matrix = matrix_sort(data);

    let mut idx = 0;
    let last_idx = data.as_ptr() as usize + n;
    let last_col: Vec<u8> = matrix.into_iter().enumerate().map(|(i, row)| {
        if row as usize == data.as_ptr() as usize {
            idx = i as u32;
        }
        let mut row = row as usize + n - 1;
        if row >= last_idx { row -= n; }
        unsafe { *(row as *const u8) }
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

pub fn naive_matrix_sort(data: &[u8]) -> Vec<u32> {
    let n = data.len();
    let mut matrix = (0..n as u32).collect::<Vec<_>>();

    matrix.sort_unstable_by(|a, b| {
        let mut a = *a as usize;
        let mut b = *b as usize;

        for _ in 0..n {
            if a >= n { a = 0; }
            if b >= n { b = 0; }
            match data[a].cmp(&data[b]) {
                Equal => {}
                non_eq => return non_eq,
            }
            a += 1;
            b += 1;
        }

        Equal
    });

    matrix
}

pub fn matrix_sort(data: &[u8]) -> Vec<*const u8> {
    type Ptr = *const u8;

    #[inline]
    fn ix(a: &[Ptr], i: usize, d: isize, n: usize, b: usize) -> u8 {
        unsafe {
            let mut ptr = a[i].offset(d) as usize;
            if ptr >= b { ptr -= n; }
            *(ptr as Ptr)
        }
    }

    fn pivot(a: &mut [Ptr], d: isize, s: usize, b: usize) -> u8 {
        fn median(a: u8, b: u8, c: u8) -> u8 {
            a.max(b).min(c)
        }

        fn med3(a: &mut [Ptr], d: isize, s: usize, b: usize) -> u8 {
            let li = 0;
            let mi = a.len() / 2;
            let ri = a.len() - 1;

            let l = ix(a, li, d, s, b);
            let m = ix(a, mi, d, s, b);
            let r = ix(a, ri, d, s, b);

            median(l, m, r)
        }

        med3(a, d, s, b)
    }

    fn partition(a: &mut [Ptr], d: isize, p: u8, s: usize, e: usize) -> (usize, usize) {
        let mut i = 0;
        let mut j = 0;
        let mut n = a.len() - 1;

        while j <= n {
            match (ix(a, j, d, s, e)).cmp(&p) {
                Less => {
                    a.swap(i, j);
                    i += 1;
                    j += 1;
                }
                Equal => {
                    j += 1;
                }
                Greater => {
                    a.swap(j, n);
                    n -= 1;
                }
            }
        }

        (i, j)
    }

    fn isort(a: &mut [Ptr], d: isize, s: usize, e: usize) {

        fn cmp(a: Ptr, b: Ptr, mut d: isize, s: usize, e: usize) -> Ordering {
            while (d as usize) < s {
                match ix(&mut [a], 0, d, s, e).cmp(&ix(&mut [b], 0, d, s, e)) {
                    Equal => {}
                    non_eq => return non_eq,
                }
                d += 1;
            }

            Equal
        }

        let mut i = 1;
        while i < a.len() {
            let mut j = i;
            while j > 0 && cmp(a[j - 1], a[j], d, s, e) == Greater {
                a.swap(j, j - 1);
                j -= 1;
            }
            i += 1;
        }
    }

    fn sort_(a: &mut [Ptr], d: isize, s: usize, e: usize) {
        if d as usize >= s { return; }
        if a.len() < 10 { return isort(a, d, s, e) }
        let p = pivot(a, d, s, e);
        let (i, j) = partition(a, d, p, s, e);
        sort_(&mut a[..i], d, s, e);
        sort_(&mut a[i..j], d + 1, s, e);
        sort_(&mut a[j..], d, s, e);
    }

    unsafe {
        let n = data.len();
        let base = data.as_ptr();
        let mut matrix = (0..n).map(|i| base.offset(i as isize)).collect::<Vec<_>>();
        sort_(&mut matrix[..], 0, n, base as usize + n);

        matrix
    }
}

#[cfg(test)]
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

#[cfg(test)]
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

        #[test]
        fn test_multi_key_quicksort(ref data in bytes_regex(".+").unwrap()) {
            let test_data = matrix_sort(data).into_iter().map(|e| unsafe {*e}).collect::<Vec<_>>();
            let reference_data = naive_matrix_sort(data).into_iter().map(|e| data[e as usize]).collect::<Vec<_>>();;
            prop_assert_eq!(test_data, reference_data);
        }

        #[test]
        fn test_initial_rle(ref data in bytes_regex(".+").unwrap()) {
            let mut buffer = Vec::with_capacity(data.len());
            let read = rle::initial_encode(data, &mut buffer);
            let decoded = rle::initial_decode(&buffer);
            prop_assert_eq!(&decoded[..], &data[..read]);
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
