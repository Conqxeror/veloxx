use criterion::{criterion_group, criterion_main, Criterion};
use indexmap::IndexMap;
use rand::prelude::*;

use veloxx::dataframe::DataFrame;
use veloxx::lazy::{binary_op, col, lit, Aggregation, BinaryOperator, LazyDataFrame};
use veloxx::series::Series;
use veloxx::types::Value;

// Helper to generate TPC-H lineitem-like data
fn generate_lineitem(rows: usize) -> DataFrame {
    let mut rng = StdRng::seed_from_u64(42);

    // l_quantity: 1-50
    let quantity: Vec<Option<f64>> = (0..rows).map(|_| Some(rng.gen_range(1.0..50.0))).collect();
    // l_extendedprice: 1.0 - 10000.0
    let extendedprice: Vec<Option<f64>> = (0..rows)
        .map(|_| Some(rng.gen_range(1.0..10000.0)))
        .collect();
    // l_discount: 0.0 - 0.1
    let discount: Vec<Option<f64>> = (0..rows).map(|_| Some(rng.gen_range(0.0..0.1))).collect();
    // l_tax: 0.0 - 0.08
    let tax: Vec<Option<f64>> = (0..rows).map(|_| Some(rng.gen_range(0.0..0.08))).collect();
    // l_returnflag: 'A', 'N', 'R'
    let flags = ["A", "N", "R"];
    let returnflag: Vec<Option<String>> = (0..rows)
        .map(|_| Some(flags[rng.gen_range(0..3)].to_string()))
        .collect();
    // l_linestatus: 'O', 'F'
    let statuses = ["O", "F"];
    let linestatus: Vec<Option<String>> = (0..rows)
        .map(|_| Some(statuses[rng.gen_range(0..2)].to_string()))
        .collect();
    // l_shipdate: days from epoch (simulated)
    let shipdate: Vec<Option<i32>> = (0..rows)
        .map(|_| Some(rng.gen_range(10000..11000)))
        .collect();

    let mut cols = IndexMap::new();
    cols.insert(
        "l_quantity".to_string(),
        Series::new_f64("l_quantity", quantity),
    );
    cols.insert(
        "l_extendedprice".to_string(),
        Series::new_f64("l_extendedprice", extendedprice),
    );
    cols.insert(
        "l_discount".to_string(),
        Series::new_f64("l_discount", discount),
    );
    cols.insert("l_tax".to_string(), Series::new_f64("l_tax", tax));
    cols.insert(
        "l_returnflag".to_string(),
        Series::new_string("l_returnflag", returnflag),
    );
    cols.insert(
        "l_linestatus".to_string(),
        Series::new_string("l_linestatus", linestatus),
    );
    cols.insert(
        "l_shipdate".to_string(),
        Series::new_i32("l_shipdate", shipdate),
    );

    DataFrame::new(cols)
}

// TPC-H Q1 (Simplified)
// SELECT
//     l_returnflag,
//     l_linestatus,
//     sum(l_quantity) as sum_qty,
//     sum(l_extendedprice) as sum_base_price,
//     count(*) as count_order
// FROM
//     lineitem
// WHERE
//     l_shipdate <= 10900
// GROUP BY
//     l_returnflag,
//     l_linestatus
fn tpch_q1(df: &DataFrame) {
    let lazy = LazyDataFrame::from_dataframe(df.clone());

    let filtered = lazy.filter(binary_op(
        col("l_shipdate"),
        BinaryOperator::LtEq,
        lit(Value::I32(10900)),
    ));

    let grouped = filtered.group_by(vec!["l_returnflag".to_string(), "l_linestatus".to_string()]);

    let aggregated = grouped.agg(vec![
        Aggregation::Sum("l_quantity".to_string()),
        Aggregation::Sum("l_extendedprice".to_string()),
        Aggregation::Count("l_returnflag".to_string()), // Proxy for count(*)
    ]);

    let _res = aggregated.collect().unwrap();
}

// TPC-H Q6 (Simplified)
// SELECT
//     sum(l_extendedprice * l_discount) as revenue
// FROM
//     lineitem
// WHERE
//     l_shipdate >= 10000
//     AND l_shipdate < 11000
//     AND l_discount >= 0.05
//     AND l_discount <= 0.07
//     AND l_quantity < 24
fn tpch_q6(df: &DataFrame) {
    // Manual execution for now as Lazy doesn't support complex arithmetic projections efficiently yet
    // Filter
    let ship_mask = df
        .get_column("l_shipdate")
        .unwrap()
        .gt(&Series::new_i32("lit", vec![Some(9999); df.row_count()]))
        .unwrap()
        .and(
            &Series::new_i32("lit", vec![Some(11000); df.row_count()])
                .gt(df.get_column("l_shipdate").unwrap())
                .unwrap(),
        )
        .unwrap();

    let disc_mask = df
        .get_column("l_discount")
        .unwrap()
        .gt(&Series::new_f64("lit", vec![Some(0.049); df.row_count()]))
        .unwrap()
        .and(
            &Series::new_f64("lit", vec![Some(0.071); df.row_count()])
                .gt(df.get_column("l_discount").unwrap())
                .unwrap(),
        )
        .unwrap();

    let qty_mask = Series::new_f64("lit", vec![Some(24.0); df.row_count()])
        .gt(df.get_column("l_quantity").unwrap())
        .unwrap();

    let final_mask = ship_mask.and(&disc_mask).unwrap().and(&qty_mask).unwrap();

    let filtered = df.filter_by_mask(&final_mask).unwrap();

    // Compute revenue: sum(price * discount)
    let price = filtered.get_column("l_extendedprice").unwrap();
    let discount = filtered.get_column("l_discount").unwrap();

    let revenue_series = price.arrow_mul(discount).unwrap(); // Use arrow_mul directly
    let _revenue = revenue_series.sum().unwrap();
}

fn bench_tpch(c: &mut Criterion) {
    let rows = 500_000;
    let df = generate_lineitem(rows);

    let mut group = c.benchmark_group("tpch");
    group.sample_size(10); // Heavy benchmark

    group.bench_function("q1_simplified", |b| b.iter(|| tpch_q1(&df)));

    group.bench_function("q6_simplified", |b| b.iter(|| tpch_q6(&df)));

    group.finish();
}

criterion_group!(benches, bench_tpch);
criterion_main!(benches);
