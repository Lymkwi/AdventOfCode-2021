extern crate common;
extern crate day01;
extern crate day02;
use criterion::{criterion_group, criterion_main, Criterion};
use common::read_data;

pub fn criterion_benchmark(c: &mut Criterion) {
    let data = read_data("day01/input").unwrap();
    c.bench_function("day01-1", |b| b.iter(|| day01::solve_part_one(&data)));
    c.bench_function("day01-2", |b| b.iter(|| day01::solve_part_two(&data)));
    let data = read_data("day02/input").unwrap();
    c.bench_function("day02-1", |b| b.iter(|| day02::solve_part_one(&data)));
    c.bench_function("day02-2", |b| b.iter(|| day02::solve_part_two(&data)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);