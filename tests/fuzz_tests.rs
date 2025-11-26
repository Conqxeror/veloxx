#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use proptest::prelude::*;
    use veloxx::dataframe::{join::JoinType, DataFrame};
    use veloxx::series::Series;
    use veloxx::types::Value;

    // Helper to create DataFrame from proptest generated data
    fn create_df(id_col: &str, ids: Vec<i32>, val_col: &str, values: Vec<i32>) -> DataFrame {
        let mut cols = IndexMap::new();
        cols.insert(
            id_col.to_string(),
            Series::new_i32(id_col, ids.into_iter().map(Some).collect()),
        );
        cols.insert(
            val_col.to_string(),
            Series::new_i32(val_col, values.into_iter().map(Some).collect()),
        );
        DataFrame::new(cols)
    }

    proptest! {
        #[test]
        fn fuzz_join_inner(
            left_ids in prop::collection::vec(0..100i32, 0..50),
            right_ids in prop::collection::vec(0..100i32, 0..50),
            left_vals in prop::collection::vec(0..1000i32, 0..50),
            right_vals in prop::collection::vec(0..1000i32, 0..50),
        ) {
            // Ensure value vecs match id vecs length
            let l_len = left_ids.len();
            let r_len = right_ids.len();

            let left_values = left_vals.into_iter().cycle().take(l_len).collect();
            let right_values = right_vals.into_iter().cycle().take(r_len).collect();

            let left_df = create_df("id", left_ids.clone(), "val_l", left_values);
            let right_df = create_df("id", right_ids.clone(), "val_r", right_values);

            // Perform Inner Join
            let joined = left_df.join(&right_df, "id", JoinType::Inner);

            prop_assert!(joined.is_ok());
            let res_df = joined.unwrap();

            // Property: Result row count should match manual calculation
            let mut expected_count = 0;
            for l_id in &left_ids {
                for r_id in &right_ids {
                    if l_id == r_id {
                        expected_count += 1;
                    }
                }
            }

            prop_assert_eq!(res_df.row_count(), expected_count);
        }

        #[test]
        fn fuzz_groupby_sum(
            group_keys in prop::collection::vec(0..10i32, 1..100),
            values in prop::collection::vec(0..100i32, 1..100),
        ) {
            let len = group_keys.len();
            let vals_adjusted: Vec<f64> = values.into_iter().cycle().take(len).map(|x| x as f64).collect();

            // Create manual map for verification
            let mut expected_sums = std::collections::HashMap::new();
            for (key, val) in group_keys.iter().zip(vals_adjusted.iter()) {
                *expected_sums.entry(*key).or_insert(0.0) += val;
            }

            let mut cols = IndexMap::new();
            cols.insert("key".to_string(), Series::new_i32("key", group_keys.into_iter().map(Some).collect()));
            cols.insert("val".to_string(), Series::new_f64("val", vals_adjusted.into_iter().map(Some).collect()));

            let df = DataFrame::new(cols);
            let grouped = df.group_by(vec!["key".to_string()]).unwrap();
            let aggregated = grouped.agg(vec![("val", "sum")]).unwrap();

            // Verify sums
            let key_series = aggregated.get_column("key").unwrap();
            let sum_series = aggregated.get_column("val_sum").unwrap();

            for i in 0..aggregated.row_count() {
                let k = key_series.get_i32(i).unwrap();
                let s = sum_series.get_f64(i).unwrap();

                let expected = expected_sums.get(&k).unwrap();
                // F64 equality might be fuzzy, but exact integer sums should be close
                prop_assert!((s - expected).abs() < 1e-9);
            }
        }
    }
}
