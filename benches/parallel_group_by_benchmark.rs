//! Benchmark for parallel group_by implementation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;
use veloxx::performance::ParallelGroupBy;

fn create_test_data(size: usize) -> Vec<i32> {
    (0..size as i32).collect()
}

fn bench_sequential_group_by(c: &mut Criterion) {
    let data = create_test_data(100000);

    c.bench_function("sequential_group_by", |b| {
        b.iter(|| {
            let mut groups = std::collections::HashMap::new();
            for &item in black_box(&data) {
                let key = item % 10;
                groups.entry(key).or_insert_with(Vec::new).push(item);
            }
            black_box(groups);
        })
    });
}

fn bench_parallel_group_by(c: &mut Criterion) {
    let data = create_test_data(100000);

    c.bench_function("parallel_group_by", |b| {
        b.iter(|| {
            let result = data.parallel_group_by(|&x| x % 10).unwrap();
            black_box(result.groups);
        })
    });
}

fn bench_rayon_group_by(c: &mut Criterion) {
    let data = create_test_data(100000);

    c.bench_function("rayon_group_by", |b| {
        b.iter(|| {
            let groups: std::collections::HashMap<i32, Vec<i32>> = data
                .par_iter()
                .fold(
                    || std::collections::HashMap::new(),
                    |mut acc, &item| {
                        let key = item % 10;
                        acc.entry(key).or_insert_with(Vec::new).push(item);
                        acc
                    },
                )
                .reduce(
                    || std::collections::HashMap::new(),
                    |mut map1, map2| {
                        for (key, mut values) in map2 {
                            map1.entry(key).or_insert_with(Vec::new).append(&mut values);
                        }
                        map1
                    },
                );
            black_box(groups);
        })
    });
}

criterion_group!(
    benches,
    bench_sequential_group_by,
    bench_parallel_group_by,
    bench_rayon_group_by,
);

criterion_main!(benches);
