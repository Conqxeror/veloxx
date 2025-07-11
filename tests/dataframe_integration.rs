mod tests {
    use std::collections::BTreeMap;
    use veloxx::dataframe::join::JoinType;
    use veloxx::dataframe::DataFrame;
    use veloxx::error::VeloxxError;
    use veloxx::expressions::Expr;
    use veloxx::series::Series;
    use veloxx::types::Value;

    #[test]
    fn test_dataframe_with_column() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "a".to_string(),
            Series::new_i32("a", vec![Some(1), Some(2), Some(3)]),
        );
        columns.insert(
            "b".to_string(),
            Series::new_i32("b", vec![Some(4), Some(5), Some(6)]),
        );
        let df = DataFrame::new(columns).unwrap();

        // Create a new column "c" as a + b
        let expr = Expr::Add(
            Box::new(Expr::Column("a".to_string())),
            Box::new(Expr::Column("b".to_string())),
        );
        let new_df = df.with_column("c", &expr).unwrap();

        assert_eq!(new_df.column_count(), 3);
        assert!(new_df.column_names().contains(&&"c".to_string()));

        let col_c = new_df.get_column("c").unwrap();
        match col_c {
            Series::I32(_, data) => {
                assert_eq!(data, &vec![Some(5), Some(7), Some(9)]);
            }
            _ => panic!("Expected I32 series for column 'c'"),
        }

        // Test creating a column with a literal value
        let expr_literal = Expr::Literal(Value::I32(10));
        let new_df_literal = df.with_column("d", &expr_literal).unwrap();
        let col_d = new_df_literal.get_column("d").unwrap();
        match col_d {
            Series::I32(_, data) => {
                assert_eq!(data, &vec![Some(10), Some(10), Some(10)]);
            }
            _ => panic!("Expected I32 series for column 'd'"),
        }

        // Test error when column already exists
        let err = df.with_column("a", &expr).unwrap_err();
        assert_eq!(
            err,
            VeloxxError::InvalidOperation("Column 'a' already exists.".to_string())
        );
    }

    #[test]
    fn test_dataframe_join() {
        // Create left DataFrame
        let mut left_cols = BTreeMap::new();
        left_cols.insert(
            "id".to_string(),
            Series::new_i32("id", vec![Some(1), Some(2), Some(3)]),
        );
        left_cols.insert(
            "left_val".to_string(),
            Series::new_string(
                "left_val",
                vec![
                    Some("a".to_string()),
                    Some("b".to_string()),
                    Some("c".to_string()),
                ],
            ),
        );
        let left_df = DataFrame::new(left_cols).unwrap();

        // Create right DataFrame
        let mut right_cols = BTreeMap::new();
        right_cols.insert(
            "id".to_string(),
            Series::new_i32("id", vec![Some(2), Some(3), Some(4)]),
        );
        right_cols.insert(
            "right_val".to_string(),
            Series::new_string(
                "right_val",
                vec![
                    Some("x".to_string()),
                    Some("y".to_string()),
                    Some("z".to_string()),
                ],
            ),
        );
        let right_df = DataFrame::new(right_cols).unwrap();

        // Test Inner Join
        let inner_join_df = left_df.join(&right_df, "id", JoinType::Inner).unwrap();
        assert_eq!(inner_join_df.row_count(), 2);
        assert_eq!(inner_join_df.column_count(), 3);
        assert_eq!(
            inner_join_df.get_column("id").unwrap().get_value(0),
            Some(Value::I32(2))
        );
        assert_eq!(
            inner_join_df.get_column("left_val").unwrap().get_value(0),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(
            inner_join_df.get_column("right_val").unwrap().get_value(0),
            Some(Value::String("x".to_string()))
        );

        // Test join on non-existent column
        let err = left_df
            .join(&right_df, "non_existent", JoinType::Inner)
            .unwrap_err();
        assert_eq!(
            err,
            VeloxxError::ColumnNotFound(
                "Join column 'non_existent' not found in left DataFrame.".to_string()
            )
        );
    }

    #[test]
    fn test_dataframe_append() {
        let mut df1_cols = BTreeMap::new();
        df1_cols.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2)]),
        );
        df1_cols.insert(
            "col2".to_string(),
            Series::new_string("col2", vec![Some("a".to_string()), Some("b".to_string())]),
        );
        let df1 = DataFrame::new(df1_cols).unwrap();

        let mut df2_cols = BTreeMap::new();
        df2_cols.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(3), Some(4)]),
        );
        df2_cols.insert(
            "col2".to_string(),
            Series::new_string("col2", vec![Some("c".to_string()), Some("d".to_string())]),
        );
        let df2 = DataFrame::new(df2_cols).unwrap();

        // Test successful append
        let appended_df = df1.append(&df2).unwrap();
        assert_eq!(appended_df.row_count(), 4);
        assert_eq!(
            appended_df.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            appended_df.get_column("col1").unwrap().get_value(3),
            Some(Value::I32(4))
        );
    }

    #[test]
    fn test_series_aggregations() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.sum().unwrap(), Some(Value::I32(6)));
        assert_eq!(series_i32.count(), 3);
        assert_eq!(series_i32.min().unwrap(), Some(Value::I32(1)));
        assert_eq!(series_i32.max().unwrap(), Some(Value::I32(3)));
        assert_eq!(series_i32.mean().unwrap(), Some(Value::F64(2.0)));

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(3.5)]);
        assert_eq!(series_f64.sum().unwrap(), Some(Value::F64(7.0)));
        assert_eq!(series_f64.count(), 3);
    }

    #[test]
    fn test_series_median() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(5), Some(2), Some(4), Some(3)]);
        assert_eq!(series_i32.median().unwrap(), Some(Value::I32(3)));

        let series_i32_even = Series::new_i32("col1", vec![Some(1), Some(4), Some(2), Some(3)]);
        assert_eq!(series_i32_even.median().unwrap(), Some(Value::F64(2.5)));

        let series_f64 = Series::new_f64(
            "col2",
            vec![Some(1.0), Some(5.0), Some(2.0), Some(4.0), Some(3.0)],
        );
        assert_eq!(series_f64.median().unwrap(), Some(Value::F64(3.0)));
    }

    #[test]
    fn test_series_std_dev() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        assert_eq!(
            series_i32.std_dev().unwrap(),
            Some(Value::F64(1.5811388300841898))
        );

        let series_f64 = Series::new_f64(
            "col2",
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
        );
        assert_eq!(
            series_f64.std_dev().unwrap(),
            Some(Value::F64(1.5811388300841898))
        );

        let series_empty_i32 = Series::new_i32("col1", vec![]);
        assert_eq!(series_empty_i32.std_dev().unwrap(), None);

        let series_single_i32 = Series::new_i32("col1", vec![Some(1)]);
        assert_eq!(series_single_i32.std_dev().unwrap(), None);
    }

    #[test]
    fn test_series_unique() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(1), None, Some(3), None]);
        let unique_i32 = series_i32.unique().unwrap();
        assert!(unique_i32.len() <= 4); // Should have at most 4 unique values: 1, 2, 3, None

        let series_string = Series::new_string("col3", vec![Some("a".to_string()), Some("b".to_string()), Some("a".to_string()), None]);
        let unique_string = series_string.unique().unwrap();
        assert!(unique_string.len() <= 3); // Should have at most 3 unique values: "a", "b", None
    }
}