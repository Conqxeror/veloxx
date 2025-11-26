#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use veloxx::dataframe::DataFrame;
    use veloxx::lazy::{binary_op, col, lit, BinaryOperator, LazyDataFrame};
    use veloxx::series::Series;
    use veloxx::types::Value;

    #[test]
    fn test_lazy_filter_collect() {
        let mut columns = IndexMap::new();
        columns.insert(
            "a".to_string(),
            Series::new_i32("a", vec![Some(1), Some(2), Some(3), Some(4)]),
        );
        columns.insert(
            "b".to_string(),
            Series::new_i32("b", vec![Some(10), Some(20), Some(30), Some(40)]),
        );

        let df = DataFrame::new(columns);

        // Lazy filter: a > 2
        let lazy_df = LazyDataFrame::from_dataframe(df).filter(binary_op(
            col("a"),
            BinaryOperator::Gt,
            lit(Value::I32(2)),
        ));

        let collected = lazy_df.collect().expect("Failed to collect");

        assert_eq!(collected.row_count(), 2);

        let a = collected.get_column("a").unwrap();
        let _b = collected.get_column("b").unwrap();

        // Should have values 3, 4
        let val0 = a.get_i32(0).unwrap();
        let val1 = a.get_i32(1).unwrap();

        // Order might vary if parallel, but standard filter preserves order usually
        assert!(val0 == 3 || val0 == 4);
        assert!(val1 == 3 || val1 == 4);
        assert_ne!(val0, val1);
    }

    #[test]
    fn test_lazy_projection_collect() {
        let mut columns = IndexMap::new();
        columns.insert(
            "a".to_string(),
            Series::new_i32("a", vec![Some(1), Some(2)]),
        );
        columns.insert(
            "b".to_string(),
            Series::new_i32("b", vec![Some(10), Some(20)]),
        );

        let df = DataFrame::new(columns);

        // Lazy select: "b"
        let lazy_df = LazyDataFrame::from_dataframe(df).select(vec![col("b")]);

        let collected = lazy_df.collect().expect("Failed to collect");

        assert_eq!(collected.column_count(), 1);
        assert!(collected.get_column("b").is_some());
        assert!(collected.get_column("a").is_none());
    }
}
