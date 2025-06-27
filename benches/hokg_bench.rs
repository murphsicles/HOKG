use criterion::{criterion_group, criterion_main, Criterion};
use hokg::{hokg, Config};

fn hokg_benchmark(c: &mut Criterion) {
    let config = Config {
        p: 5,  // Small prime
        a: 1,  // Curve parameter a
        b: 1,  // Curve parameter b
        x0: 2, // Seed x-coordinate
        y0: 3, // Seed y-coordinate
        k: 2,  // Lifting exponent
    };

    c.bench_function("hokg", |b| b.iter(|| hokg(config.clone())));
}

criterion_group!(benches, hokg_benchmark);
criterion_main!(benches);
