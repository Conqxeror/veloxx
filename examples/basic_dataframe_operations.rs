use std::collections::BTreeMap;
use veloxx::conditions::Condition;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use veloxx::types::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. DataFrame Creation
    println!("--- DataFrame Creation ---");
    let mut columns = BTreeMap::new();
    columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("David".to_string()),
            ],
        ),
    );
    columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(30), Some(24), Some(35), Some(29)]),
    );
    columns.insert(
        "city".to_string(),
        Series::new_string(
            "city",
            vec![
                Some("New York".to_string()),
                Some("London".to_string()),
                Some("New York".to_string()),
                Some("London".to_string()),
            ],
        ),
    );
    let df = DataFrame::new(columns)?;
    println!("Initial DataFrame:\n{}", df);

    // 2. Filtering a DataFrame
    println!("\n--- Filtering DataFrame (age > 30) ---");
    let condition = Condition::Gt("age".to_string(), Value::I32(30));
    let filtered_df = df.filter(&condition)?;
    println!("Filtered DataFrame:\n{}", filtered_df);

    println!("\n--- Filtering DataFrame (city == 'New York') ---");
    let condition = Condition::Eq("city".to_string(), Value::String("New York".to_string()));
    let filtered_df_city = df.filter(&condition)?;
    println!("Filtered DataFrame:\n{}", filtered_df_city);

    // 3. Aggregation
    println!("\n--- Aggregating by City (count of people) ---");
    let grouped_df = df.group_by(vec!["city".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![("age", "count")])?;
    println!(
        "Aggregated DataFrame (Count of Age by City):\n{}",
        aggregated_df
    );

    println!("\n--- Aggregating by City (average age) ---");
    let grouped_df = df.group_by(vec!["city".to_string()])?;
    let aggregated_df = grouped_df.agg(vec![("age", "mean")])?;
    println!(
        "Aggregated DataFrame (Mean Age by City):\n{}",
        aggregated_df
    );

    // 4. Selecting Columns
    println!("\n--- Selecting Columns (name, age) ---");
    let selected_df = df.select_columns(vec!["name".to_string(), "age".to_string()])?;
    println!("Selected Columns DataFrame:\n{}", selected_df);

    // 5. Dropping Columns
    println!("\n--- Dropping Column (city) ---");
    let dropped_df = df.drop_columns(vec!["city".to_string()])?;
    println!("Dropped Column DataFrame:\n{}", dropped_df);

    // 6. Renaming a Column
    println!("\n--- Renaming Column (age to years) ---");
    let renamed_df = df.rename_column("age", "years")?;
    println!("Renamed Column DataFrame:\n{}", renamed_df);

    // 7. Sorting DataFrame
    println!("\n--- Sorting DataFrame by Age (ascending) ---");
    let sorted_df = df.sort(vec!["age".to_string()], true)?;
    println!("Sorted DataFrame:\n{}", sorted_df);

    // 8. Appending DataFrames
    println!("\n--- Appending DataFrames ---");
    let mut columns_more = BTreeMap::new();
    columns_more.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![Some("Eve".to_string()), Some("Frank".to_string())],
        ),
    );
    columns_more.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(22), Some(40)]),
    );
    columns_more.insert(
        "city".to_string(),
        Series::new_string(
            "city",
            vec![Some("Paris".to_string()), Some("London".to_string())],
        ),
    );
    let df_more = DataFrame::new(columns_more)?;
    let appended_df = df.append(&df_more)?;
    println!("Appended DataFrame:\n{}", appended_df);

    Ok(())
}
