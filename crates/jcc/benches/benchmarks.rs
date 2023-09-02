use criterion::BenchmarkId;

//use criterion::{criterion_group, criterion_main};
use criterion::{black_box, criterion_group, criterion_main, Criterion}; //
use jcc::convert;

pub fn small_policy_config(c: &mut Criterion) {
    let test_config: String = std::fs::read_to_string("tests/data/config_4.txt").unwrap();
    c.bench_function("small_policy_config", |b| {
        b.iter(|| {
            // Code to be benchmarked
            let _result = convert(black_box(&test_config));
        });
    });
}

pub fn full_switch_config(c: &mut Criterion) {
    let test_config: String = std::fs::read_to_string("tests/data/config_17.txt").unwrap();
    c.bench_function("full_switch_config", |b| {
        b.iter(|| {
            // Code to be benchmarked
            let _result = convert(black_box(&test_config));
        });
    });
}

criterion_group!(benches, full_switch_config, small_policy_config);

criterion_main!(benches);
