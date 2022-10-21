use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{distributions::{Alphanumeric, DistString}, thread_rng, Rng};

pub fn password_gen_first(pwd_amount: usize, pwd_size: usize) -> Vec<String>{
    let mut password_vec = vec![];
    for _ in 0..pwd_amount {
        let string = Alphanumeric.sample_string(&mut thread_rng(), pwd_size);
        password_vec.push(string);
    }
    password_vec
}
pub fn password_gen_second(pwd_amount: usize, pwd_size: usize) -> Vec<String>{
    let mut password_vec = vec![];
    for _ in 0..pwd_amount {
        let string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(pwd_size)
        .map(char::from)
        .collect();
        password_vec.push(string);
    }
    password_vec
}
pub fn password_gen_third(pwd_amount: usize, pwd_size: usize) -> Vec<String>{
    let mut password_vec = vec![];
    for _ in 0..pwd_amount {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let string: String = (0..pwd_size)
        .map(|_| {
            let index = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();
    password_vec.push(string);
    }
    password_vec
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Password Gen");
    group.sample_size(100);
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("500 pass lenght 16 first", |b| b.iter(|| password_gen_first(black_box(500), black_box(16))));
    group.bench_function("500 pass lenght 16 second", |b| b.iter(|| password_gen_second(black_box(500), black_box(16))));
    group.bench_function("500 pass lenght 16 third", |b| b.iter(|| password_gen_third(black_box(500), black_box(16))));
    // group.bench_function("500000 pass lenght 256 first", |b| b.iter(|| password_gen_third(black_box(500000), black_box(256))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);