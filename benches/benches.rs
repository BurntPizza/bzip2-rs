
extern crate bzip2_rs;
// extern crate criterion;

// use criterion::{Bencher, Criterion, Benchmark, ParameterizedBenchmark, Throughput};

use std::time::*;

const BWT_SIZE: usize = 100_000;

fn bench<F: Fn() -> Vec<u8>>(name: &str, size: usize, f: F) {
    println!("\n{}", name);
    let mut _v = vec![];

    let mut time = Duration::from_secs(0);
    let mut iterations = 0;
    loop {
        let start = Instant::now();
        _v = f();
        time += start.elapsed();
        iterations += 1;
        if time > Duration::from_secs(3) { break; }
    }
    let s = time.as_secs() as f64 + (time.subsec_nanos() as f64 / 1_000_000_000.0);
    let ti = s / iterations as f64;
    let mut tif = format!("{:.3} s", ti);
    if ti < 1.0 {
        tif = format!("{:.3} ms", ti * 1000.0);
    }
    println!("Iterations: {}", iterations);
    println!("T/I: {}", tif);
    let t = (size as f64 / (1024 * 1024) as f64) / ti;
    let tf = if t >= 1.0 { format!("{:.3} Mb/s", t) } else { format!("{:.3} Kb/s", t * 1024.0) };
    println!("Throughput: {}", tf);
}

fn bench_bwt_mkqs_text() {
    let mut enc = bzip2_rs::rle::Encoder::new(BWT_SIZE);
    let data = text_data(BWT_SIZE);
    while enc.encode(&data) != 0 {}
    let data = enc.finish();

    bench("BWT_mkqs_text", BWT_SIZE, || bzip2_rs::bwt(&data).0);
}


fn bench_bwt_mkqs_binary() {
    let mut enc = bzip2_rs::rle::Encoder::new(BWT_SIZE);
    let data = binary_data(BWT_SIZE);
    while enc.encode(&data) != 0 {}
    let data = enc.finish();

    bench("BWT_mkqs_binary", BWT_SIZE, || bzip2_rs::bwt(&data).0);
}

fn bench_bwt_sa_naive_text() {
    let mut enc = bzip2_rs::rle::Encoder::new(BWT_SIZE);
    let data = text_data(BWT_SIZE);
    while enc.encode(&data) != 0 {}
    let data = enc.finish();

    bench("bwt_sa_naive_text", BWT_SIZE, || bzip2_rs::bwt_sa_naive(&data).0);
}


fn bench_bwt_sa_naive_binary() {
    let mut enc = bzip2_rs::rle::Encoder::new(BWT_SIZE);
    let data = binary_data(BWT_SIZE);
    while enc.encode(&data) != 0 {}
    let data = enc.finish();

    bench("bwt_sa_naive_binary", BWT_SIZE, || bzip2_rs::bwt_sa_naive(&data).0);
}


fn text_data(size: usize) -> Vec<u8> {
    TEXT.iter().cloned().cycle().take(size).collect()
}

fn binary_data(size: usize) -> Vec<u8> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open("/usr/bin/wget").unwrap();
    let f_size = f.metadata().unwrap().len() as usize;
    let mut v = Vec::with_capacity(size);
    unsafe { v.set_len(size); }
    f.read_exact(&mut v[..size.min(f_size)]).unwrap();
    if f_size < size {
        v = v.into_iter().cycle().take(size).collect();
    }
    v
}

pub fn histogram(data: &[u8]) {
    let sum: u64 = data.iter().map(|b| *b as u64).sum();
    let pdf: Vec<f64> = data.iter().map(|c| *c as f64 / sum as f64).collect();
    let mut max = 0.0;
    for p in &pdf {
        if max < *p { max = *p; }
    }
    for i in 0..256 {
        let n = (pdf[i] / max * 100.0).round() as usize;
        if n > 0 {
            println!("{:3}: {:5.2}% | {}", i, pdf[i], ::std::iter::repeat('#').take(n).collect::<String>());
        }
    }
}

fn main() {
    bench_bwt_sa_naive_text();
    bench_bwt_sa_naive_binary();

    bench_bwt_mkqs_text();
    bench_bwt_mkqs_binary();

    // Criterion::default()
    //     .bench("bwt_mkqs", Benchmark::new("text", bench_bwt_mkqs_text)
    //            .throughput(Throughput::Bytes(BWT_SIZE as _))
    //     .bench("bwt_mkqs", Benchmark::new("binary", bench_bwt_mkqs_binary)
    //            .throughput(Throughput::Bytes(BWT_SIZE as _))
    //     ;

    
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
