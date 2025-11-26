#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use veloxx::dataframe::{DataFrame, Pivot};
    use veloxx::series::Series;

    #[test]
    fn test_pivot_basic() {
        let mut columns = IndexMap::new();
        columns.insert(
            "date".to_string(),
            Series::new_string(
                "date",
                vec![
                    Some("2021-01-01".to_string()),
                    Some("2021-01-01".to_string()),
                    Some("2021-01-02".to_string()),
                    Some("2021-01-02".to_string()),
                ],
            ),
        );
        columns.insert(
            "city".to_string(),
            Series::new_string(
                "city",
                vec![
                    Some("London".to_string()),
                    Some("Paris".to_string()),
                    Some("London".to_string()),
                    Some("Paris".to_string()),
                ],
            ),
        );
        columns.insert(
            "temp".to_string(),
            Series::new_f64("temp", vec![Some(10.0), Some(12.0), Some(11.0), Some(13.0)]),
        );

        let df = DataFrame::new(columns);

        // Pivot: index="date", columns="city", values="temp"
        let pivoted = df
            .pivot("temp", vec!["date".to_string()], "city", "mean")
            .expect("Pivot failed");

        println!("{}", pivoted);

        assert_eq!(pivoted.row_count(), 2);
        assert_eq!(pivoted.column_count(), 3); // date, London, Paris

        // Check schema
        let cols = pivoted.column_names();
        assert!(cols.contains(&"date".to_string()));
        assert!(cols.contains(&"London".to_string()));
        assert!(cols.contains(&"Paris".to_string()));

        // Check values
        // Row 1 (2021-01-01): London=10, Paris=12
        // Row 2 (2021-01-02): London=11, Paris=13

        // We can't easily guarantee row order without sorting, but let's check existence
        // Since implementation sorts by first appearance or hash map iteration, let's find the row.

        // Helper to find row by date
        let date_series = pivoted.get_column("date").unwrap();
        let london_series = pivoted.get_column("London").unwrap();

        let idx_01 = (0..2)
            .find(|&i| date_series.get_string(i).unwrap() == "2021-01-01")
            .unwrap();
        assert_eq!(london_series.get_f64(idx_01).unwrap(), 10.0);
    }

    #[test]
    fn test_pivot_multi_index() {
        let mut columns = IndexMap::new();
        columns.insert(
            "A".to_string(),
            Series::new_string(
                "A",
                vec![
                    Some("foo".to_string()),
                    Some("foo".to_string()),
                    Some("bar".to_string()),
                ],
            ),
        );
        columns.insert(
            "B".to_string(),
            Series::new_string(
                "B",
                vec![
                    Some("one".to_string()),
                    Some("two".to_string()),
                    Some("one".to_string()),
                ],
            ),
        );
        columns.insert(
            "C".to_string(),
            Series::new_string(
                "C",
                vec![
                    Some("small".to_string()),
                    Some("small".to_string()),
                    Some("large".to_string()),
                ],
            ),
        );
        columns.insert(
            "D".to_string(),
            Series::new_i32("D", vec![Some(1), Some(2), Some(3)]),
        );

        let df = DataFrame::new(columns);

        // Pivot: index=["A", "B"], columns="C", values="D"
        let pivoted = df
            .pivot("D", vec!["A".to_string(), "B".to_string()], "C", "sum")
            .unwrap();

        // Expected rows: (foo, one), (foo, two), (bar, one)
        // Cols: A, B, small, large
        assert_eq!(pivoted.column_count(), 4);
        assert_eq!(pivoted.row_count(), 3);

        let _a_series = pivoted.get_column("A").unwrap();
        let _small_series = pivoted.get_column("small").unwrap();

        // Find (foo, one)
        // foo -> one -> small=1
        // foo -> two -> small=2
        // bar -> one -> large=3 (small=null)
    }
}
