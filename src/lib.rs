
#[cfg(test)]
extern crate bzip2;
#[cfg(test)]
#[macro_use]
extern crate proptest;



#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use bzip2::Compression;
    use bzip2::read::*;

    use proptest::string::*;

    use *;

    type Bwt = (Vec<u8>, u32);

    fn bwt_ref(input: &[u8]) -> Bwt {
        use std::collections::VecDeque;

        if input.is_empty() {
            return (vec![], 0);
        }

        let mut matrix = Vec::with_capacity(input.len());

        let mut row: VecDeque<u8> = input.iter().cloned().collect();;
        for i in 0..input.len() {
            let b = row.pop_back().unwrap();
            row.push_front(b);
            matrix.push(row.clone());
        }
        debug_assert_eq!(matrix.len(), input.len());
        matrix.sort();

        let idx = matrix.iter().position(|row| &*row.iter().cloned().collect::<Vec<_>>() == input).unwrap();
        let mut last_col = matrix.into_iter().map(|row| row[row.len() - 1]).collect();

        (last_col, idx as u32)
    }

    fn ibwt_ref(bwt: Bwt) -> Vec<u8> {
        use std::collections::VecDeque;

        let (last_col, idx) = bwt;
        let n = last_col.len();

        if last_col.is_empty() {
            return vec![];
        }

        let mut matrix = (0..n).map(|_| VecDeque::with_capacity(n)).collect::<Vec<_>>();

        for _ in 0..n {
            for (row, ch) in matrix.iter_mut().zip(&last_col) {
                row.push_front(*ch);
            }
            matrix.sort();
        }

        std::mem::replace(&mut matrix[idx as usize], Default::default()).into()
    }
    
    proptest! {
        #[test]
        fn bwt_reference_round_trip(ref data in bytes_regex(".*").unwrap()) {
            let bwt = bwt_ref(&data[..]);
            let round_trip = ibwt_ref(bwt);
            prop_assert_eq!(data, &round_trip)
        }
    }

    // proptest! {
    //     #[test]
    //     fn test(ref data in bytes_regex(".*").unwrap()) {
    //         let enc = ReadEncoder::new(&data[..]);
    //         let mut bz = BzEncoder::new(&data[..], Compression::Best);
    //         let mut out = vec![];
    //         let mut bz_out = vec![];
    //         enc.read_to_end(&mut out).unwrap();
    //         bz.read_to_end(&mut bz_out).unwrap();
    //         assert_eq!(out, bz_out);

    //         let mut dec = ReadDecoder::new(&out[..]);
    //         let mut bz_dec = BzDecoder::new(&bz_out[..]);
    //         let mut round_trip = vec![];
    //         let mut bz_round_trip = vec![];
    //         dec.read_to_end(&mut round_trip).unwrap();
    //         bz_dec.read_to_end(&mut bz_round_trip).unwrap();
    //         assert_eq!(round_trip, bz_round_trip);
    //         assert_eq!(&data[..], &round_trip[..]);
    //     }
    // }

    #[test]
    fn it_works() {
        let data = "Hello World!".as_bytes();
        let mut encoder = BzEncoder::new(data, Compression::Best);
        let mut compressed = vec![];
        encoder.read_to_end(&mut compressed).unwrap();
        println!("raw: {:?}", compressed);
        println!("hex: {}", compressed.iter().map(|b| format!("{:X}", b)).collect::<String>());
        println!("ascii: {}", compressed.iter().map(|b| format!("{}", (*b as char).escape_default())).collect::<String>());
    }
}
