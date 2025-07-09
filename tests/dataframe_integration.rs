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
        assert_eq!(
            inner_join_df.get_column("id").unwrap().get_value(1),
            Some(Value::I32(3))
        );
        assert_eq!(
            inner_join_df.get_column("left_val").unwrap().get_value(1),
            Some(Value::String("c".to_string()))
        );
        assert_eq!(
            inner_join_df.get_column("right_val").unwrap().get_value(1),
            Some(Value::String("y".to_string()))
        );

        // Test Left Join
        let left_join_df = left_df.join(&right_df, "id", JoinType::Left).unwrap();
        assert_eq!(left_join_df.row_count(), 3);
        assert_eq!(left_join_df.column_count(), 3);
        assert_eq!(
            left_join_df.get_column("id").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            left_join_df.get_column("left_val").unwrap().get_value(0),
            Some(Value::String("a".to_string()))
        );
        assert_eq!(
            left_join_df.get_column("right_val").unwrap().get_value(0),
            None
        );
        assert_eq!(
            left_join_df.get_column("id").unwrap().get_value(1),
            Some(Value::I32(2))
        );
        assert_eq!(
            left_join_df.get_column("right_val").unwrap().get_value(1),
            Some(Value::String("x".to_string()))
        );
        assert_eq!(
            left_join_df.get_column("id").unwrap().get_value(2),
            Some(Value::I32(3))
        );
        assert_eq!(
            left_join_df.get_column("right_val").unwrap().get_value(2),
            Some(Value::String("y".to_string()))
        );

        // Test Right Join
        let right_join_df = left_df.join(&right_df, "id", JoinType::Right).unwrap();
        assert_eq!(right_join_df.row_count(), 3);
        assert_eq!(right_join_df.column_count(), 3);
        assert_eq!(
            right_join_df.get_column("id").unwrap().get_value(0),
            Some(Value::I32(2))
        );
        assert_eq!(
            right_join_df.get_column("left_val").unwrap().get_value(0),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(
            right_join_df.get_column("id").unwrap().get_value(1),
            Some(Value::I32(3))
        );
        assert_eq!(
            right_join_df.get_column("left_val").unwrap().get_value(1),
            Some(Value::String("c".to_string()))
        );
        assert_eq!(
            right_join_df.get_column("id").unwrap().get_value(2),
            Some(Value::I32(4))
        );
        assert_eq!(
            right_join_df.get_column("left_val").unwrap().get_value(2),
            None
        );
        assert_eq!(
            right_join_df.get_column("right_val").unwrap().get_value(2),
            Some(Value::String("z".to_string()))
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
        assert_eq!(
            appended_df.get_column("col2").unwrap().get_value(0),
            Some(Value::String("a".to_string()))
        );
        assert_eq!(
            appended_df.get_column("col2").unwrap().get_value(3),
            Some(Value::String("d".to_string()))
        );

        // Test append with different number of columns
        let mut df3_cols = BTreeMap::new();
        df3_cols.insert("col1".to_string(), Series::new_i32("col1", vec![Some(5)]));
        let df3 = DataFrame::new(df3_cols).unwrap();
        let err = df1.append(&df3).unwrap_err();
        assert_eq!(
            err,
            VeloxxError::InvalidOperation(
                "Cannot append DataFrames with different number of columns.".to_string()
            )
        );

        // Test append with different column names
        let mut df4_cols = BTreeMap::new();
        df4_cols.insert("col1".to_string(), Series::new_i32("col1", vec![Some(5)]));
        df4_cols.insert(
            "col3".to_string(),
            Series::new_string("col3", vec![Some("e".to_string())]),
        );
        let df4 = DataFrame::new(df4_cols).unwrap();
        let err = df1.append(&df4).unwrap_err();
        assert_eq!(
            err,
            VeloxxError::InvalidOperation(
                "Cannot append DataFrames with different column names or order.".to_string()
            )
        );

        // Test append with mismatched data types
        let mut df5_cols = BTreeMap::new();
        df5_cols.insert("col1".to_string(), Series::new_f64("col1", vec![Some(5.0)]));
        df5_cols.insert(
            "col2".to_string(),
            Series::new_string("col2", vec![Some("e".to_string())]),
        );
        let df5 = DataFrame::new(df5_cols).unwrap();
        let err = df1.append(&df5).unwrap_err();
        assert_eq!(
            err,
            VeloxxError::DataTypeMismatch(
                "Cannot append DataFrames with mismatched data types for column 'col1'."
                    .to_string()
            )
        );
    }
    #[test]
    fn test_series_sum() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.sum().unwrap(), Some(Value::I32(6)));

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(3.5)]);
        assert_eq!(series_f64.sum().unwrap(), Some(Value::F64(7.0)));

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.sum().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_series_count() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.count(), 3);

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(3.5)]);
        assert_eq!(series_f64.count(), 3);

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert_eq!(series_bool.count(), 2);

        let series_string = Series::new_string(
            "col4",
            vec![Some("a".to_string()), None, Some("b".to_string())],
        );
        assert_eq!(series_string.count(), 2);
    }

    #[test]
    fn test_series_min() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.min().unwrap(), Some(Value::I32(1)));

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(0.5)]);
        assert_eq!(series_f64.min().unwrap(), Some(Value::F64(0.5)));

        let series_empty_i32 = Series::new_i32("col1", vec![]);
        assert_eq!(series_empty_i32.min().unwrap(), None);

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.min().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_series_max() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.max().unwrap(), Some(Value::I32(3)));

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(0.5)]);
        assert_eq!(series_f64.max().unwrap(), Some(Value::F64(2.5)));

        let series_empty_i32 = Series::new_i32("col1", vec![]);
        assert_eq!(series_empty_i32.max().unwrap(), None);

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.max().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_series_mean() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]);
        assert_eq!(series_i32.mean().unwrap(), Some(Value::F64(2.0)));

        let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.0), None, Some(3.0)]);
        assert_eq!(series_f64.mean().unwrap(), Some(Value::F64(2.0)));

        let series_empty_i32 = Series::new_i32("col1", vec![]);
        assert_eq!(series_empty_i32.mean().unwrap(), None);

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.mean().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
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

        let series_f64_even =
            Series::new_f64("col2", vec![Some(1.0), Some(4.0), Some(2.0), Some(3.0)]);
        assert_eq!(series_f64_even.median().unwrap(), Some(Value::F64(2.5)));

        let series_empty_i32 = Series::new_i32("col1", vec![]);
        assert_eq!(series_empty_i32.median().unwrap(), None);

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.median().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
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

        let series_bool = Series::new_bool("col3", vec![Some(true), Some(false), None]);
        assert!(matches!(
            series_bool.std_dev().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    // #[test]
    // fn test_series_unique() {
    //     let series_i32 = Series::new_i32("col1", vec![Some(1), Some(2), Some(1), None, Some(3), None]);
    //     let unique_i32 = series_i32.unique().unwrap();
    //     let mut actual_i32_values: Vec<Option<Value>> = (0..unique_i32.len()).map(|i| unique_i32.get_value(i)).collect();
    //     actual_i32_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     let mut expected_i32_values = vec![None, Some(Value::I32(1)), Some(Value::I32(2)), Some(Value::I32(3))];
    //     expected_i32_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     assert_eq!(actual_i32_values, expected_i32_values);

    //     let series_f64 = Series::new_f64("col2", vec![Some(1.0), Some(2.5), Some(1.0), None, Some(3.5), None]);
    //     let unique_f64 = series_f64.unique().unwrap();
    //     let mut actual_f64_values: Vec<Option<Value>> = (0..unique_f64.len()).map(|i| unique_f64.get_value(i)).collect();
    //     actual_f64_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     let mut expected_f64_values = vec![None, Some(Value::F64(1.0)), Some(Value::F64(2.5)), Some(Value::F64(3.5))];
    //     expected_f64_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     assert_eq!(actual_f64_values, expected_f64_values);

    //     let series_string = Series::new_string("col3", vec![Some("a".to_string()), Some("b".to_string()), Some("a".to_string()), None, Some("c".to_string()), None]);
    //     let unique_string = series_string.unique().unwrap();
    //     let mut actual_string_values: Vec<Option<Value>> = (0..unique_string.len()).map(|i| unique_string.get_value(i)).collect();
    //     actual_string_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     let mut expected_string_values = vec![None, Some(Value::String("a".to_string())), Some(Value::String("b".to_string())), Some(Value::String("c".to_string()))];
    //     expected_string_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     assert_eq!(actual_string_values, expected_string_values);

    //     let series_bool = Series::new_bool("col4", vec![Some(true), Some(false), Some(true), None]);
    //     let unique_bool = series_bool.unique().unwrap();
    //     let mut actual_bool_values: Vec<Option<Value>> = (0..unique_bool.len()).map(|i| unique_bool.get_value(i)).collect();
    //     actual_bool_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     let mut expected_bool_values = vec![None, Some(Value::Bool(false)), Some(Value::Bool(true))];
    //     expected_bool_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    //     assert_eq!(actual_bool_values, expected_bool_values);
    // }

    #[test]
    fn test_series_unique_simple_f64() {
        let series_f64 = Series::new_f64(
            "col1",
            vec![
                Some(1.0),
                Some(2.5),
                Some(1.0),
                None,
                Some(3.5),
                None,
                Some(f64::NAN),
                Some(-0.0),
                Some(0.0),
            ],
        );
        let unique_f64 = series_f64.unique().unwrap();
        let mut actual_f64_values: Vec<Option<Value>> = (0..unique_f64.len())
            .map(|i| unique_f64.get_value(i))
            .collect();
        actual_f64_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut expected_f64_values = vec![
            None,
            Some(Value::F64(1.0)),
            Some(Value::F64(2.5)),
            Some(Value::F64(3.5)),
            Some(Value::F64(f64::NAN)),
            Some(Value::F64(-0.0)),
            Some(Value::F64(0.0)),
        ];
        expected_f64_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        assert_eq!(actual_f64_values.len(), expected_f64_values.len());
        for i in 0..actual_f64_values.len() {
            match (&actual_f64_values[i], &expected_f64_values[i]) {
                (Some(Value::F64(a)), Some(Value::F64(b))) if a.is_nan() && b.is_nan() => {
                    // NaN comparison is special, they are equal if both are NaN
                    assert!(a.is_nan() && b.is_nan());
                }
                (a, b) => assert_eq!(a, b),
            }
        }
    }

    #[test]
    fn test_dataframe_group_by_and_agg() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "city".to_string(),
            Series::new_string(
                "city",
                vec![Some("London".to_string()), Some("Paris".to_string())],
            ),
        );
        columns.insert(
            "age".to_string(),
            Series::new_i32("age", vec![Some(10), Some(20)]),
        );
        let df = DataFrame::new(columns).unwrap();

        let grouped_df = df.group_by(vec!["city".to_string()]).unwrap();

        // Test sum aggregation
        let aggregated_sum = grouped_df.agg(vec![("age", "sum")]).unwrap();
        let city_col = aggregated_sum.get_column("city").unwrap();
        let age_sum_col = aggregated_sum.get_column("age_sum").unwrap();

        let mut found_london = false;
        let mut found_paris = false;

        for i in 0..aggregated_sum.row_count() {
            let city = city_col.get_value(i).unwrap();
            let age_sum = age_sum_col.get_value(i).unwrap();

            if city == Value::String("London".to_string()) {
                assert_eq!(age_sum, Value::I32(10));
                found_london = true;
            } else if city == Value::String("Paris".to_string()) {
                assert_eq!(age_sum, Value::I32(20));
                found_paris = true;
            } else {
                panic!("Unexpected city in aggregated results");
            }
        }
        assert!(found_london && found_paris);

        // Test count aggregation
        let aggregated_count = grouped_df.agg(vec![("age", "count")]).unwrap();
        let city_col = aggregated_count.get_column("city").unwrap();
        let age_count_col = aggregated_count.get_column("age_count").unwrap();

        let mut found_london = false;
        let mut found_paris = false;

        for i in 0..aggregated_count.row_count() {
            let city = city_col.get_value(i).unwrap();
            let age_count = age_count_col.get_value(i).unwrap();

            if city == Value::String("London".to_string()) {
                assert_eq!(age_count, Value::I32(1));
                found_london = true;
            } else if city == Value::String("Paris".to_string()) {
                assert_eq!(age_count, Value::I32(1));
                found_paris = true;
            } else {
                panic!("Unexpected city in aggregated results");
            }
        }
        assert!(found_london && found_paris);
    }

    #[test]
    fn test_dataframe_describe() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2), Some(3), None]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_f64("col2", vec![Some(1.0), Some(2.5), None, Some(3.5)]),
        );
        columns.insert(
            "col3".to_string(),
            Series::new_bool("col3", vec![Some(true), Some(false), None, Some(true)]),
        );
        let df = DataFrame::new(columns).unwrap();

        let described_df = df.describe().unwrap();

        assert_eq!(described_df.row_count(), 3);
        assert_eq!(described_df.column_count(), 7);

        assert_eq!(
            described_df.get_column("column").unwrap().get_value(0),
            Some(Value::String("col1".to_string()))
        );
        assert_eq!(
            described_df.get_column("count").unwrap().get_value(0),
            Some(Value::I32(3))
        );
        assert_eq!(
            described_df.get_column("mean").unwrap().get_value(0),
            Some(Value::F64(2.0))
        );
        assert!(
            (match described_df
                .get_column("std")
                .unwrap()
                .get_value(0)
                .unwrap()
            {
                Value::F64(val) => val,
                _ => panic!("Expected F64 value"),
            } - 1.0)
                .abs()
                < 1e-9
        );
        assert_eq!(
            described_df.get_column("min").unwrap().get_value(0),
            Some(Value::String("I32(1)".to_string()))
        );
        assert_eq!(
            described_df.get_column("max").unwrap().get_value(0),
            Some(Value::String("I32(3)".to_string()))
        );
        assert_eq!(
            described_df.get_column("median").unwrap().get_value(0),
            Some(Value::String("I32(2)".to_string()))
        );

        assert_eq!(
            described_df.get_column("column").unwrap().get_value(1),
            Some(Value::String("col2".to_string()))
        );
        assert_eq!(
            described_df.get_column("count").unwrap().get_value(1),
            Some(Value::I32(3))
        );
        assert_eq!(
            described_df.get_column("mean").unwrap().get_value(1),
            Some(Value::F64(2.3333333333333335))
        );
        assert!(
            (match described_df
                .get_column("std")
                .unwrap()
                .get_value(1)
                .unwrap()
            {
                Value::F64(val) => val,
                _ => panic!("Expected F64 value"),
            } - 1.258305739211791)
                .abs()
                < 1e-9
        );
        assert_eq!(
            described_df.get_column("min").unwrap().get_value(1),
            Some(Value::String("F64(1.0)".to_string()))
        );
        assert_eq!(
            described_df.get_column("max").unwrap().get_value(1),
            Some(Value::String("F64(3.5)".to_string()))
        );
        assert_eq!(
            described_df.get_column("median").unwrap().get_value(1),
            Some(Value::String("F64(2.5)".to_string()))
        );

        assert_eq!(
            described_df.get_column("column").unwrap().get_value(2),
            Some(Value::String("col3".to_string()))
        );
        assert_eq!(
            described_df.get_column("count").unwrap().get_value(2),
            Some(Value::I32(3))
        );
        assert_eq!(described_df.get_column("mean").unwrap().get_value(2), None);
        assert_eq!(described_df.get_column("std").unwrap().get_value(2), None);
        assert_eq!(described_df.get_column("min").unwrap().get_value(2), None);
        assert_eq!(described_df.get_column("max").unwrap().get_value(2), None);
        assert_eq!(
            described_df.get_column("median").unwrap().get_value(2),
            None
        );
    }

    #[test]
    fn test_dataframe_to_vec_of_vec() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2), None]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![
                    Some("a".to_string()),
                    Some("b".to_string()),
                    Some("c".to_string()),
                ],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        let vec_of_vec = df.to_vec_of_vec();

        assert_eq!(vec_of_vec.len(), 3);
        assert_eq!(vec_of_vec[0].len(), 2);
        assert_eq!(vec_of_vec[0][0], Some(Value::I32(1)));
        assert_eq!(vec_of_vec[0][1], Some(Value::String("a".to_string())));
        assert_eq!(vec_of_vec[1][0], Some(Value::I32(2)));
        assert_eq!(vec_of_vec[1][1], Some(Value::String("b".to_string())));
        assert_eq!(vec_of_vec[2][0], None);
        assert_eq!(vec_of_vec[2][1], Some(Value::String("c".to_string())));
    }

    #[test]
    fn test_dataframe_to_csv() {
        let mut columns = BTreeMap::new();
        columns.insert(
            "col1".to_string(),
            Series::new_i32("col1", vec![Some(1), Some(2), None]),
        );
        columns.insert(
            "col2".to_string(),
            Series::new_string(
                "col2",
                vec![
                    Some("a".to_string()),
                    Some("b".to_string()),
                    Some("c".to_string()),
                ],
            ),
        );
        let df = DataFrame::new(columns).unwrap();

        let test_file = "test_output.csv";
        df.to_csv(test_file).unwrap();

        let content = std::fs::read_to_string(test_file).unwrap();
        let expected_content = "col1,col2\n1,a\n2,b\n,c\n";
        assert_eq!(content, expected_content);

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_series_correlation() {
        let series_x_i32 = Series::new_i32("x", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let series_y_i32 = Series::new_i32("y", vec![Some(2), Some(4), Some(5), Some(4), Some(5)]);
        let corr_i32 = series_x_i32.correlation(&series_y_i32).unwrap().unwrap();
        assert!(matches!(corr_i32, Value::F64(_)));
        if let Value::F64(val) = corr_i32 {
            println!("Actual corr_i32: {}", val);
            assert!((val - 0.7745966692414834).abs() < 1e-8);
        }

        let series_x_f64 = Series::new_f64(
            "x",
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
        );
        let series_y_f64 = Series::new_f64(
            "y",
            vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)],
        );
        let corr_f64 = series_x_f64.correlation(&series_y_f64).unwrap().unwrap();
        assert!(matches!(corr_f64, Value::F64(_)));
        if let Value::F64(val) = corr_f64 {
            assert!((val - 0.7745966692414834).abs() < 1e-8);
        }

        let series_x_mixed =
            Series::new_i32("x", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let series_y_mixed = Series::new_f64(
            "y",
            vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)],
        );
        let corr_mixed = series_x_mixed
            .correlation(&series_y_mixed)
            .unwrap()
            .unwrap();
        assert!(matches!(corr_mixed, Value::F64(_)));
        if let Value::F64(val) = corr_mixed {
            assert!((val - 0.7745966692414834).abs() < 1e-8);
        }

        let series_empty = Series::new_i32("empty", vec![]);
        assert_eq!(
            series_x_i32.correlation(&series_empty).unwrap_err(),
            VeloxxError::InvalidOperation(
                "Series must have the same length for correlation calculation.".to_string()
            )
        );
        assert_eq!(
            series_empty.correlation(&series_x_i32).unwrap_err(),
            VeloxxError::InvalidOperation(
                "Series must have the same length for correlation calculation.".to_string()
            )
        );

        let series_single = Series::new_i32("single", vec![Some(1)]);
        assert_eq!(
            series_x_i32.correlation(&series_single).unwrap_err(),
            VeloxxError::InvalidOperation(
                "Series must have the same length for correlation calculation.".to_string()
            )
        );

        let series_non_numeric_x = Series::new_bool("x", vec![Some(true), Some(false), Some(true)]);
        let series_non_numeric_y = Series::new_i32("y", vec![Some(1), Some(2), Some(3)]);
        assert!(matches!(
            series_non_numeric_x
                .correlation(&series_non_numeric_y)
                .unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_series_covariance() {
        let series_x_i32 = Series::new_i32("x", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let series_y_i32 = Series::new_i32("y", vec![Some(2), Some(4), Some(5), Some(4), Some(5)]);
        let cov_i32 = series_x_i32.covariance(&series_y_i32).unwrap().unwrap();
        assert!(matches!(cov_i32, Value::F64(_)));
        if let Value::F64(val) = cov_i32 {
            assert!((val - 1.5).abs() < 1e-9);
        }

        let series_x_f64 = Series::new_f64(
            "x",
            vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0), Some(5.0)],
        );
        let series_y_f64 = Series::new_f64(
            "y",
            vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)],
        );
        let cov_f64 = series_x_f64.covariance(&series_y_f64).unwrap().unwrap();
        assert!(matches!(cov_f64, Value::F64(_)));
        if let Value::F64(val) = cov_f64 {
            assert!((val - 1.5).abs() < 1e-9);
        }

        let series_x_mixed =
            Series::new_i32("x", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let series_y_mixed = Series::new_f64(
            "y",
            vec![Some(2.0), Some(4.0), Some(5.0), Some(4.0), Some(5.0)],
        );
        let cov_mixed = series_x_mixed.covariance(&series_y_mixed).unwrap().unwrap();
        assert!(matches!(cov_mixed, Value::F64(_)));
        if let Value::F64(val) = cov_mixed {
            assert!((val - 1.5).abs() < 1e-9);
        }

        let series_empty = Series::new_i32("empty", vec![]);
        assert_eq!(
            series_x_i32.covariance(&series_empty).unwrap_err(),
            VeloxxError::InvalidOperation(
                "Series must have the same length for covariance calculation.".to_string()
            )
        );
        assert_eq!(
            series_empty.covariance(&series_x_i32).unwrap_err(),
            VeloxxError::InvalidOperation(
                "Series must have the same length for covariance calculation.".to_string()
            )
        );

        let series_single = Series::new_i32("single", vec![Some(1)]);
        assert_eq!(series_single.covariance(&series_single).unwrap(), None);

        let series_non_numeric_x = Series::new_bool("x", vec![Some(true), Some(false), Some(true)]);
        let series_non_numeric_y = Series::new_i32("y", vec![Some(1), Some(2), Some(3)]);
        assert!(matches!(
            series_non_numeric_x
                .covariance(&series_non_numeric_y)
                .unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_dataframe_from_csv() {
        use std::io::Write;
        let test_file = "test_input.csv";
        let csv_content = "col1,col2,col3\n1,a,true\n2,b,false\n3,c,true";
        let mut file = std::fs::File::create(test_file).unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();

        let df = DataFrame::from_csv(test_file).unwrap();

        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 3);

        assert_eq!(
            df.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            df.get_column("col2").unwrap().get_value(1),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(
            df.get_column("col3").unwrap().get_value(2),
            Some(Value::Bool(true))
        );

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_series_interpolate_nulls() {
        let series_i32 = Series::new_i32("col1", vec![Some(1), None, Some(3), None, Some(5)]);
        let interpolated_i32 = series_i32.interpolate_nulls().unwrap();
        assert_eq!(interpolated_i32.get_value(0), Some(Value::I32(1)));
        assert_eq!(interpolated_i32.get_value(1), Some(Value::I32(2)));
        assert_eq!(interpolated_i32.get_value(2), Some(Value::I32(3)));
        assert_eq!(interpolated_i32.get_value(3), Some(Value::I32(4)));
        assert_eq!(interpolated_i32.get_value(4), Some(Value::I32(5)));

        let series_f64 = Series::new_f64(
            "col2",
            vec![Some(1.0), None, None, Some(4.0), None, Some(6.0)],
        );
        let interpolated_f64 = series_f64.interpolate_nulls().unwrap();
        assert_eq!(interpolated_f64.get_value(0), Some(Value::F64(1.0)));
        assert_eq!(interpolated_f64.get_value(1), Some(Value::F64(2.0)));
        assert_eq!(interpolated_f64.get_value(2), Some(Value::F64(3.0)));
        assert_eq!(interpolated_f64.get_value(3), Some(Value::F64(4.0)));
        assert_eq!(interpolated_f64.get_value(4), Some(Value::F64(5.0)));
        assert_eq!(interpolated_f64.get_value(5), Some(Value::F64(6.0)));

        let series_no_nulls = Series::new_i32("col3", vec![Some(1), Some(2), Some(3)]);
        let interpolated_no_nulls = series_no_nulls.interpolate_nulls().unwrap();
        assert_eq!(interpolated_no_nulls.get_value(0), Some(Value::I32(1)));
        assert_eq!(interpolated_no_nulls.get_value(1), Some(Value::I32(2)));
        assert_eq!(interpolated_no_nulls.get_value(2), Some(Value::I32(3)));

        let series_all_nulls = Series::new_i32("col4", vec![None, None, None]);
        let interpolated_all_nulls = series_all_nulls.interpolate_nulls().unwrap();
        assert_eq!(interpolated_all_nulls.get_value(0), None);
        assert_eq!(interpolated_all_nulls.get_value(1), None);
        assert_eq!(interpolated_all_nulls.get_value(2), None);

        let series_start_null = Series::new_i32("col5", vec![None, Some(2), Some(3)]);
        let interpolated_start_null = series_start_null.interpolate_nulls().unwrap();
        assert_eq!(interpolated_start_null.get_value(0), None);
        assert_eq!(interpolated_start_null.get_value(1), Some(Value::I32(2)));
        assert_eq!(interpolated_start_null.get_value(2), Some(Value::I32(3)));

        let series_end_null = Series::new_i32("col6", vec![Some(1), Some(2), None]);
        let interpolated_end_null = series_end_null.interpolate_nulls().unwrap();
        assert_eq!(interpolated_end_null.get_value(0), Some(Value::I32(1)));
        assert_eq!(interpolated_end_null.get_value(1), Some(Value::I32(2)));
        assert_eq!(interpolated_end_null.get_value(2), None);

        let series_bool = Series::new_bool("col7", vec![Some(true), None, Some(false)]);
        assert!(matches!(
            series_bool.interpolate_nulls().unwrap_err(),
            VeloxxError::Unsupported(_)
        ));
    }

    #[test]
    fn test_dataframe_from_vec_of_vec_source() {
        use veloxx::dataframe::sources::DataFrameSource;
        let data = vec![
            vec!["col1".to_string(), "col2".to_string(), "col3".to_string()],
            vec!["1".to_string(), "a".to_string(), "true".to_string()],
            vec!["2".to_string(), "b".to_string(), "false".to_string()],
            vec!["3".to_string(), "c".to_string(), "true".to_string()],
        ];

        let df = data.to_dataframe().unwrap();

        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 3);

        assert_eq!(
            df.get_column("col1").unwrap().get_value(0),
            Some(Value::I32(1))
        );
        assert_eq!(
            df.get_column("col2").unwrap().get_value(1),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(
            df.get_column("col3").unwrap().get_value(2),
            Some(Value::Bool(true))
        );
    }

    #[test]
    fn test_dataframe_from_json() {
        let df = DataFrame::from_json("examples/test.json").unwrap();
        assert_eq!(df.row_count(), 3);
        assert_eq!(df.column_count(), 3);
        assert_eq!(
            df.get_column("col1").unwrap().get_value(0),
            Some(Value::F64(1.0))
        );
        assert_eq!(
            df.get_column("col2").unwrap().get_value(1),
            Some(Value::String("b".to_string()))
        );
        assert_eq!(df.get_column("col3").unwrap().get_value(2), None);
    }
}
