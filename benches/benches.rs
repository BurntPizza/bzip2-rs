
extern crate bzip2_rs;
extern crate criterion;

use criterion::{Bencher, Criterion, ParameterizedBenchmark, Throughput};

fn bench_ibwt(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();
    let (transformed, start) = bzip2_rs::bwt(&data[..]);

    b.iter(|| {
        bzip2_rs::ibwt(&transformed[..], start)
    })
}

fn bench_bwt(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();

    b.iter(|| {
        bzip2_rs::bwt(&data[..])
    })
}

fn bench_naive_matrix_sort(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();

    b.iter(|| {
        bzip2_rs::naive_matrix_sort(&data[..])
    })
}

fn bench_matrix_sort(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();

    b.iter(|| {
        bzip2_rs::matrix_sort(&data[..])
    })
}

fn bench_initial_rle_encode(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();

    b.iter(|| {
        bzip2_rs::rle::initial_encode(&data[..])
    })
}

fn bench_initial_rle_decode(b: &mut Bencher, size: &usize) {
    let data: Vec<u8> = TEXT.iter().cloned().cycle().take(*size).collect();
    let data: Vec<u8> = bzip2_rs::initial_rle_encode(&data);

    b.iter(|| {
        bzip2_rs::rle::initial_decode(&data[..])
    })
}

fn main() {
    Criterion::default()
        .bench("initial_rle_encode", ParameterizedBenchmark::new("initial_rle_encode", bench_initial_rle_encode, vec![100_000])
               .throughput(|n| Throughput::Bytes(*n as u32)))
        // .bench("initial_rle_decode", ParameterizedBenchmark::new("initial_rle_decode", bench_initial_rle_decode, vec![100_000])
        //        .throughput(|n| Throughput::Bytes(*n as u32)))

        // .bench("bwt", ParameterizedBenchmark::new("bwt", bench_bwt, vec![100, 1000])
        //        .throughput(|n| Throughput::Bytes(*n as u32)))
        // .bench("ibwt", ParameterizedBenchmark::new("ibwt", bench_ibwt, vec![1000, 10_000])
        //        .throughput(|n| Throughput::Bytes(*n as u32)))

        // .bench("naive_matrix_sort", ParameterizedBenchmark::new("naive_matrix_sort", bench_naive_matrix_sort, vec![100, 1000])
        //        .throughput(|n| Throughput::Bytes(*n as u32)))
        // .bench("matrix_sort", ParameterizedBenchmark::new("matrix_sort", bench_matrix_sort, vec![100, 1000])
        //        .throughput(|n| Throughput::Bytes(*n as u32)))
        ;
}

const TEXT: &[u8] = b"
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
 Quisque accumsan sagittis mattis.
 Morbi tempor odio vitae orci tempor, maximus blandit sapien tincidunt.
 Pellentesque eleifend odio eget lacus facilisis vehicula.
 Quisque mattis, arcu quis malesuada fermentum, libero purus pharetra ante, vitae elementum erat arcu vel neque.
 Donec porttitor, turpis in auctor porttitor, quam urna blandit magna, in rhoncus nibh lorem in erat.
 Nam et ligula leo.
 Pellentesque congue dolor eget nisi viverra malesuada.
 Quisque magna purus, eleifend et augue nec, suscipit sagittis mi.
 Nunc nec blandit nibh.


Aliquam pharetra iaculis massa, eget dignissim dui rhoncus sit amet.
 Praesent finibus lobortis malesuada.
 Phasellus quis convallis diam, sit amet finibus augue.
 Cras lobortis efficitur nunc at vehicula.
 In ligula lectus, faucibus id finibus tincidunt, lacinia semper tellus.
 Donec accumsan libero erat.
 Aenean aliquet lorem diam, ut convallis odio consectetur sit amet.


Nulla luctus sagittis velit scelerisque dapibus.
 Donec id sodales nulla, nec ornare diam.
 Vestibulum elit sem, maximus porta purus vitae, egestas ultrices lectus.
 Vivamus id pharetra libero.
 Suspendisse pretium, dui eu porttitor consequat, ante tortor egestas ante, euismod aliquet elit nulla sed neque.
 Nam eleifend porta fringilla.
 Suspendisse aliquam purus eros, in euismod mi convallis ac.
 Mauris mattis malesuada porttitor.
 Ut consectetur maximus ex vehicula blandit.
 Mauris bibendum dictum arcu.
 Mauris a orci sed ipsum sodales tincidunt sed pellentesque erat.


Phasellus posuere tellus ac est hendrerit posuere quis et odio.
 Fusce viverra, enim id laoreet finibus, purus est semper neque, id accumsan elit tellus ut sem.
 Aliquam aliquam metus et congue placerat.
 Suspendisse lobortis, enim vel scelerisque faucibus, nunc ex porttitor orci, at fringilla augue tellus sit amet turpis.
 Maecenas posuere est quis placerat luctus.
 Phasellus feugiat risus velit, eu dignissim nisl elementum nec.
 Aliquam et augue at mauris condimentum imperdiet.


Nulla tincidunt tempor leo et venenatis.
 Aenean a lorem laoreet, ultricies metus id, viverra diam.
 In gravida fermentum odio id facilisis.
 Aliquam ac ante eleifend, pulvinar urna in, sollicitudin ipsum.
 Pellentesque nec malesuada massa.
 Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.
 Donec molestie a nisl sit amet bibendum.
 Aenean at elementum quam, elementum consequat ipsum.
 Fusce lobortis lectus ante, et vestibulum mauris molestie vitae.
 Ut iaculis elit vel lorem euismod auctor.
 Ut sed turpis orci.
 Interdum et malesuada fames ac ante ipsum primis in faucibus.
 Vivamus mattis cursus lacinia.
 Maecenas eget malesuada dui.
 Etiam dignissim elementum mi id laoreet.";
