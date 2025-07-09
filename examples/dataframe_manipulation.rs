use std::collections::BTreeMap;
use veloxx::conditions::Condition;
use veloxx::dataframe::DataFrame;
use veloxx::expressions::Expr;
use veloxx::series::Series;
use veloxx::types::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initial DataFrame
    println!("--- 1. Initial DataFrame ---");
    let mut columns = BTreeMap::new();
    columns.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(1), Some(2), Some(3), Some(4), Some(5)]),
    );
    columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("David".to_string()),
                Some("Eve".to_string()),
            ],
        ),
    );
    columns.insert(
        "age".to_string(),
        Series::new_i32(
            "age",
            vec![Some(30), Some(24), Some(35), Some(29), Some(42)],
        ),
    );
    columns.insert(
        "city".to_string(),
        Series::new_string(
            "city",
            vec![
                Some("New York".to_string()),
                Some("London".to_string()),
                Some("New York".to_string()),
                Some("Paris".to_string()),
                Some("London".to_string()),
            ],
        ),
    );
    columns.insert(
        "salary".to_string(),
        Series::new_f64(
            "salary",
            vec![
                Some(50000.0),
                Some(60000.0),
                Some(75000.0),
                Some(55000.0),
                Some(80000.0),
            ],
        ),
    );
    let df = DataFrame::new(columns)?;
    println!("{}", df);
    // 2. Select Columns
    println!("\n--- 2. Select Columns (name, age) ---");
    let selected_df = df.select_columns(vec!["name".to_string(), "age".to_string()])?;
    println!("{}", selected_df);
    // 3. Drop Columns
    println!("\n--- 3. Drop Columns (salary) ---");
    let dropped_df = df.drop_columns(vec!["salary".to_string()])?;
    println!("{}", dropped_df);
    // 4. Rename Column
    println!("\n--- 4. Rename Column (age to years_old) ---");
    let renamed_df = df.rename_column("age", "years_old")?;
    println!("{}", renamed_df);
    // 5. Sort DataFrame (by age ascending)
    println!("\n--- 5. Sort DataFrame (by age ascending) ---");
    let sorted_df = df.sort(vec!["age".to_string()], true)?;
    println!("{}", sorted_df);
    // 6. Add a new column (bonus_salary = salary * 0.1)
    println!("\n--- 6. Add a new column (bonus_salary = salary * 0.1) ---");
    let bonus_expr = Expr::Multiply(
        Box::new(Expr::Column("salary".to_string())),
        Box::new(Expr::Literal(Value::F64(0.1))),
    );
    let df_with_bonus = df.with_column("bonus_salary", &bonus_expr)?;
    println!("{}", df_with_bonus);
    // 7. Filter DataFrame (age > 30 and city == "New York")
    println!("\n--- 7. Filter DataFrame (age > 30 and city == \"New York\") ---");
    let condition = Condition::And(
        Box::new(Condition::Gt("age".to_string(), Value::I32(30))),
        Box::new(Condition::Eq(
            "city".to_string(),
            Value::String("New York".to_string()),
        )),
    );
    let filtered_df = df.filter(&condition)?;
    println!("{}", filtered_df);
    // 8. Append DataFrames
    println!("\n--- 8. Append DataFrames ---");
    let mut new_columns = BTreeMap::new();
    new_columns.insert(
        "id".to_string(),
        Series::new_i32("id", vec![Some(6), Some(7)]),
    );
    new_columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![Some("Frank".to_string()), Some("Grace".to_string())],
        ),
    );
    new_columns.insert(
        "age".to_string(),
        Series::new_i32("age", vec![Some(22), Some(38)]),
    );
    new_columns.insert(
        "city".to_string(),
        Series::new_string(
            "city",
            vec![Some("Berlin".to_string()), Some("London".to_string())],
        ),
    );
    new_columns.insert(
        "salary".to_string(),
        Series::new_f64("salary", vec![Some(45000.0), Some(70000.0)]),
    );
    let df2 = DataFrame::new(new_columns)?;
    println!("DataFrame 2:\n{}", df2);
    let appended_df = df.append(&df2)?;
    println!("Appended DataFrame:\n{}", appended_df);
    Ok(())
}
