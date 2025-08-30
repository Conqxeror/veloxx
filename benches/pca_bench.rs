#[macro_use]
extern crate criterion;
use criterion::Criterion;

fn bench_pca_first_component(c: &mut Criterion) {
    let n_samples = 10_000;
    let n_features = 50;
    let _matrix: Vec<Vec<f64>> = (0..n_samples)
        .map(|_| (0..n_features).map(|j| (j as f64) * 0.1).collect())
        .collect();
    c.bench_function("pca_first_component_simd", |b| {
        b.iter(|| {
            // PCA functionality not available
        })
    });
}

criterion_group!(benches, bench_pca_first_component);
criterion_main!(benches);
