// benches/hokg_bench.rs

use criterion::{criterion_group, criterion_main, Criterion};
use hokg::{hokg, Config};

// Benchmark for the HOKG algorithm
fn hokg_benchmark(c: &mut Criterion) {
    // Create a sample configuration for the elliptic curve
    let config = Config {
        p: 5,  // Small prime
        a: 1,  // Curve parameter a
        b: 1,  // Curve parameter b
        x0: 2, // Seed x-coordinate
        y0: 3, // Seed y-coordinate
        k: 2,  // Lifting exponent
    };

    // Benchmark the hokg function
    c.bench_function("hokg", |b| b.iter(|| hokg(config.clone())));
}

// Define the benchmark group and main function
criterion_group!(benches, hokg_benchmark);
criterion_main!(benches);
