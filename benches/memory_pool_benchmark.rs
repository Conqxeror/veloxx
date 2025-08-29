//! Benchmark comparing old and new memory pool implementations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use veloxx::performance::{AdvancedMemoryPool, MemoryPool};

fn bench_old_memory_pool_allocation(c: &mut Criterion) {
    let pool = MemoryPool::new(1024);

    c.bench_function("old_memory_pool_allocation", |b| {
        b.iter(|| {
            let block = pool.allocate(black_box(100)).unwrap();
            black_box(block.len());
            pool.deallocate(block);
        })
    });
}

fn bench_new_memory_pool_allocation(c: &mut Criterion) {
    let pool = AdvancedMemoryPool::new(1024);

    c.bench_function("new_memory_pool_allocation", |b| {
        b.iter(|| {
            let (ptr, layout) = pool.allocate(black_box(100)).unwrap();
            black_box(ptr);
            unsafe {
                pool.deallocate(ptr, layout);
            }
        })
    });
}

fn bench_old_memory_pool_reuse(c: &mut Criterion) {
    let pool = MemoryPool::new(1024);

    c.bench_function("old_memory_pool_reuse", |b| {
        b.iter(|| {
            let block1 = pool.allocate(100).unwrap();
            black_box(block1.len());
            pool.deallocate(block1);

            let block2 = pool.allocate(100).unwrap();
            black_box(block2.len());
            pool.deallocate(block2);
        })
    });
}

fn bench_new_memory_pool_reuse(c: &mut Criterion) {
    let pool = AdvancedMemoryPool::new(1024);

    c.bench_function("new_memory_pool_reuse", |b| {
        b.iter(|| {
            let (ptr1, layout1) = pool.allocate(100).unwrap();
            black_box(ptr1);
            unsafe {
                pool.deallocate(ptr1, layout1);
            }

            let (ptr2, layout2) = pool.allocate(100).unwrap();
            black_box(ptr2);
            unsafe {
                pool.deallocate(ptr2, layout2);
            }
        })
    });
}

fn bench_old_memory_pool_multiple_sizes(c: &mut Criterion) {
    let pool = MemoryPool::new(1024);

    c.bench_function("old_memory_pool_multiple_sizes", |b| {
        b.iter(|| {
            for size in [50, 100, 200, 500] {
                let block = pool.allocate(size).unwrap();
                black_box(block.len());
                pool.deallocate(block);
            }
        })
    });
}

fn bench_new_memory_pool_multiple_sizes(c: &mut Criterion) {
    let pool = AdvancedMemoryPool::new(1024);

    c.bench_function("new_memory_pool_multiple_sizes", |b| {
        b.iter(|| {
            for size in [50, 100, 200, 500] {
                let (ptr, layout) = pool.allocate(size).unwrap();
                black_box(ptr);
                unsafe {
                    pool.deallocate(ptr, layout);
                }
            }
        })
    });
}

criterion_group!(
    benches,
    bench_old_memory_pool_allocation,
    bench_new_memory_pool_allocation,
    bench_old_memory_pool_reuse,
    bench_new_memory_pool_reuse,
    bench_old_memory_pool_multiple_sizes,
    bench_new_memory_pool_multiple_sizes,
);

criterion_main!(benches);
