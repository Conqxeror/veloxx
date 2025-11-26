#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use veloxx::dataframe::{join::JoinType, DataFrame};
    use veloxx::series::Series;
    use veloxx::types::Value;

    #[test]
    fn test_outer_join() {
        let mut left_cols = IndexMap::new();
        left_cols.insert(
            "id".to_string(),
            Series::new_i32("id", vec![Some(1), Some(2)]),
        );
        left_cols.insert(
            "left_val".to_string(),
            Series::new_string(
                "left_val",
                vec![Some("L1".to_string()), Some("L2".to_string())],
            ),
        );
        let left_df = DataFrame::new(left_cols);

        let mut right_cols = IndexMap::new();
        right_cols.insert(
            "id".to_string(),
            Series::new_i32("id", vec![Some(2), Some(3)]),
        );
        right_cols.insert(
            "right_val".to_string(),
            Series::new_string(
                "right_val",
                vec![Some("R2".to_string()), Some("R3".to_string())],
            ),
        );
        let right_df = DataFrame::new(right_cols);

        let joined = left_df
            .join(&right_df, "id", JoinType::Outer)
            .expect("Outer join failed");

        // Expected IDs: 1, 2, 3
        assert_eq!(joined.row_count(), 3);

        let id_series = joined.get_column("id").unwrap();
        let left_val_series = joined.get_column("left_val").unwrap();
        let right_val_series = joined.get_column("right_val").unwrap();

        // Find ID=1 (Left only)
        // Can't rely on order, so search
        let idx_1 = (0..3)
            .find(|&i| id_series.get_i32(i) == Some(1))
            .expect("ID 1 missing");
        assert_eq!(left_val_series.get_string(idx_1).unwrap(), "L1");
        assert!(right_val_series.get_string(idx_1).is_none());

        // Find ID=2 (Both)
        let idx_2 = (0..3)
            .find(|&i| id_series.get_i32(i) == Some(2))
            .expect("ID 2 missing");
        assert_eq!(left_val_series.get_string(idx_2).unwrap(), "L2");
        assert_eq!(right_val_series.get_string(idx_2).unwrap(), "R2");

        // Find ID=3 (Right only)
        let idx_3 = (0..3)
            .find(|&i| id_series.get_i32(i) == Some(3))
            .expect("ID 3 missing");
        assert!(left_val_series.get_string(idx_3).is_none());
        assert_eq!(right_val_series.get_string(idx_3).unwrap(), "R3");
    }
}
