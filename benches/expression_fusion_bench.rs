#[macro_use]
extern crate criterion;
use criterion::Criterion;
use veloxx::performance::expression_fusion::ExpressionFusion;

fn bench_fused_add_mul(c: &mut Criterion) {
    let a_vec: Vec<f64> = (0..100_000).map(|x| x as f64).collect();
    let b_vec: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.5).collect();
    let c_vec: Vec<f64> = (0..100_000).map(|x| (x as f64) * 0.2).collect();
    let mut result = vec![0.0; a_vec.len()];
    c.bench_function("fused_add_mul_parallel", |bencher| {
        bencher.iter(|| {
            ExpressionFusion::fused_add_mul_f64(&a_vec, &b_vec, &c_vec, &mut result).unwrap();
        })
    });
}

fn bench_fused_filter_sum(c: &mut Criterion) {
    let values_vec: Vec<f64> = (0..100_000).map(|x| x as f64).collect();
    let condition: Vec<bool> = (0..100_000).map(|x| x % 2 == 0).collect();
    c.bench_function("fused_filter_sum_parallel", |bencher| {
        bencher.iter(|| {
            let _ = ExpressionFusion::fused_filter_sum_f64(&values_vec, &condition).unwrap();
        })
    });
}

criterion_group!(benches, bench_fused_add_mul, bench_fused_filter_sum);
criterion_main!(benches);
