use std::collections::HashMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a DataFrame for grouping and aggregation examples
    let mut columns = HashMap::new();
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
                Some("New York".to_string()),
            ],
        ),
    );
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            vec![
                Some("A".to_string()),
                Some("B".to_string()),
                Some("A".to_string()),
                Some("C".to_string()),
                Some("B".to_string()),
                Some("C".to_string()),
            ],
        ),
    );
    columns.insert(
        "sales".to_string(),
        Series::new_f64(
            "sales",
            vec![
                Some(100.0),
                Some(150.0),
                Some(200.0),
                Some(50.0),
                Some(120.0),
                Some(300.0),
            ],
        ),
    );
    columns.insert(
        "quantity".to_string(),
        Series::new_i32(
            "quantity",
            vec![Some(10), Some(15), Some(20), Some(5), Some(12), Some(30)],
        ),
    );
    let df = DataFrame::new(columns)?;
    println!("Original DataFrame:\n{}", df);
    // Group by 'city' and calculate sum of sales and mean of quantity
    println!("\n--- Group by 'city' (sum of sales, mean of quantity) ---");
    let grouped_by_city = df.group_by(vec!["city".to_string()])?;
    let aggregated_by_city = grouped_by_city.agg(vec![("sales", "sum"), ("quantity", "mean")])?;
    println!("{}", aggregated_by_city);
    // Group by 'city' and 'category' and calculate count of sales and min/max of quantity
    println!("\n--- Group by 'city' and 'category' (count of sales, min/max of quantity) ---");
    let grouped_by_city_category = df.group_by(vec!["city".to_string(), "category".to_string()])?;
    let aggregated_by_city_category = grouped_by_city_category.agg(vec![
        ("sales", "count"),
        ("quantity", "min"),
        ("quantity", "max"),
    ])?;
    println!("{}", aggregated_by_city_category);
    // Describe the original DataFrame
    println!("\n--- Descriptive Statistics of Original DataFrame ---");
    let described_df = df.describe()?;
    println!("{}", described_df);
    // Calculate correlation between sales and quantity
    println!("\n--- Correlation between 'sales' and 'quantity' ---");
    let correlation = df.correlation("sales", "quantity")?;
    println!("Correlation (sales, quantity): {:.4}", correlation);
    // Calculate covariance between sales and quantity
    println!("\n--- Covariance between 'sales' and 'quantity' ---");
    let covariance = df.covariance("sales", "quantity")?;
    println!("Covariance (sales, quantity): {:.4}", covariance);
    Ok(())
}
