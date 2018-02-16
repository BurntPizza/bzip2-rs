
#[cfg(test)]
extern crate bzip2;
#[cfg(test)]
#[macro_use]
extern crate proptest;

pub type Bwt = (Vec<u8>, u32);

pub fn bwt(data: &mut [u8]) -> u32 {
    unimplemented!()
}

pub fn ibwt(data: &mut [u8], start: u32) {
    let n = data.len();
    if n == 0 { return; }


}

pub fn sorted(data: &[u8]) -> Vec<u8> {
    let mut counts = [0u32; 256];

    for i in 0..data.len() {
        counts[data[i] as usize] += 1;
    }

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

#[cfg(test)]
pub fn bwt_ref(data: &mut [u8]) -> u32 {
    use std::collections::VecDeque;

    if data.is_empty() {
        return 0;
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

    for (c, d) in matrix.into_iter().map(|row| row[row.len() - 1]).zip(data.iter_mut()) {
        *d = c;
    }

    idx as u32
}

#[cfg(test)]
pub fn ibwt_ref(data: &mut [u8], start: u32) {
    use std::collections::VecDeque;

    if data.is_empty() {
        return;
    }

    let n = data.len();

    let mut matrix = (0..n).map(|_| VecDeque::with_capacity(n)).collect::<Vec<_>>();

    for _ in 0..n {
        for (row, ch) in matrix.iter_mut().zip(&mut data[..]) {
            row.push_front(*ch);
        }
        matrix.sort();
    }

    for (c, d) in matrix[start as usize].iter().zip(data) {
        *d = *c;
    }
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
            let mut scratch = data.clone();
            let idx = bwt_ref(&mut scratch[..]);
            ibwt_ref(&mut scratch[..], idx);
            prop_assert_eq!(&scratch[..], &data[..])
        }

        #[test]
        fn counting_sort(ref data in bytes_regex(".*").unwrap()) {
            let c_sorted = sorted(&data[..]);
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
