// benches/hokg_bench.rs

use criterion::{criterion_group, criterion_main, Criterion};
use hokg::{hokg, Config};

fn bench_hokg(c: &mut Criterion) {
    let config = Config {
        p: 17,
        a: 2,
        b: 3,
        x0: 5,
        y0: 6,
        k: 5,
    };

    c.bench_function("hokg_key_generation", |b| {
        b.iter(|| {
            hokg(config.clone()).expect("HOKG failed during benchmark");
        });
    });
}

criterion_group!(benches, bench_hokg);
criterion_main!(benches);
