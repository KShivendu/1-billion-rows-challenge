use criterion::{criterion_group, criterion_main, Criterion};

use one_brc::processor::process_data;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("demo_file", |b| {
        b.iter(|| process_data("data/weather_stations.csv", "data/output.csv"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
