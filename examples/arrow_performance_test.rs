// Simple timing test for ArrowSeries optimizations
use std::time::Instant;
use veloxx::arrow::series::ArrowSeries;

fn main() {
    let size = 100_000;

    // Create test data
    let data: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let opt_data: Vec<Option<f64>> = data.into_iter().map(Some).collect();

    println!("Testing ArrowSeries with {} elements", size);

    // Test ArrowSeries sum
    let series = ArrowSeries::new_f64("test", opt_data.clone());

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = series.simd_sum().unwrap();
    }
    let duration = start.elapsed();
    println!("ArrowSeries SIMD sum (1000 iterations): {:?}", duration);
    println!("Average per iteration: {:?}", duration / 1000);

    // Test ArrowSeries add
    let series1 = ArrowSeries::new_f64("test1", opt_data.clone());
    let series2 = ArrowSeries::new_f64("test2", opt_data);

    let start = Instant::now();
    for _ in 0..100 {
        let _ = series1.simd_add(&series2).unwrap();
    }
    let duration = start.elapsed();
    println!("ArrowSeries SIMD add (100 iterations): {:?}", duration);
    println!("Average per iteration: {:?}", duration / 100);
}
