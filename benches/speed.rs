use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

// Code to measure
fn my_fn(input: &str) -> usize {
    input.bytes().filter(|&b| b.is_ascii_alphabetic()).count()
}

fn speed(c: &mut Criterion) {
    c.bench_function("my_fn/small", |b| {
        b.iter(|| my_fn(black_box("Hello, Criterion! 123")));
    });
}

criterion_group!(benches, speed);
criterion_main!(benches);
