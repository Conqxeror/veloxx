//! Example demonstrating window functions and advanced analytics in Velox
//!
//! This example shows the structure for window function operations.
//! The actual implementations are placeholders until the window_functions feature is fully implemented.

use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Velox Window Functions & Advanced Analytics Example");
    println!("==================================================");

    // Create sample sales data
    let sales_df = create_sales_data()?;
    println!("Sample sales data:");
    println!("{}", sales_df);

    // Create sample time series data
    let timeseries_df = create_timeseries_data()?;
    println!("\nSample time series data:");
    println!("{}", timeseries_df);

    // Demonstrate ranking functions
    ranking_functions_example(&sales_df)?;

    // Demonstrate offset functions (lag/lead)
    offset_functions_example(&sales_df)?;

    // Demonstrate aggregate window functions
    aggregate_window_functions_example(&sales_df)?;

    // Demonstrate time-based window functions
    time_based_window_functions_example(&timeseries_df)?;

    // Demonstrate advanced analytics
    advanced_analytics_example(&sales_df)?;

    println!("\nWindow functions and advanced analytics examples completed!");
    println!("Note: Enable the 'window_functions' feature to use actual window operations:");
    println!("cargo run --example window_functions_analytics --features window_functions");

    Ok(())
}

fn create_sales_data() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut columns = BTreeMap::new();

    // Sales representative
    columns.insert(
        "sales_rep".to_string(),
        Series::new_string(
            "sales_rep",
            vec![
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
                Some("Alice".to_string()),
                Some("Bob".to_string()),
                Some("Charlie".to_string()),
            ],
        ),
    );

    // Region
    columns.insert(
        "region".to_string(),
        Series::new_string(
            "region",
            vec![
                Some("North".to_string()),
                Some("North".to_string()),
                Some("North".to_string()),
                Some("South".to_string()),
                Some("South".to_string()),
                Some("South".to_string()),
                Some("East".to_string()),
                Some("East".to_string()),
                Some("East".to_string()),
                Some("West".to_string()),
                Some("West".to_string()),
                Some("West".to_string()),
            ],
        ),
    );

    // Quarter
    columns.insert(
        "quarter".to_string(),
        Series::new_string(
            "quarter",
            vec![
                Some("Q1".to_string()),
                Some("Q1".to_string()),
                Some("Q1".to_string()),
                Some("Q1".to_string()),
                Some("Q1".to_string()),
                Some("Q1".to_string()),
                Some("Q2".to_string()),
                Some("Q2".to_string()),
                Some("Q2".to_string()),
                Some("Q2".to_string()),
                Some("Q2".to_string()),
                Some("Q2".to_string()),
            ],
        ),
    );

    // Sales amount
    columns.insert(
        "sales".to_string(),
        Series::new_f64(
            "sales",
            vec![
                Some(15000.0),
                Some(12000.0),
                Some(18000.0), // Q1 North
                Some(22000.0),
                Some(19000.0),
                Some(16000.0), // Q1 South
                Some(17000.0),
                Some(14000.0),
                Some(21000.0), // Q2 East
                Some(25000.0),
                Some(23000.0),
                Some(20000.0), // Q2 West
            ],
        ),
    );

    // Units sold
    columns.insert(
        "units".to_string(),
        Series::new_i32(
            "units",
            vec![
                Some(150),
                Some(120),
                Some(180), // Q1 North
                Some(220),
                Some(190),
                Some(160), // Q1 South
                Some(170),
                Some(140),
                Some(210), // Q2 East
                Some(250),
                Some(230),
                Some(200), // Q2 West
            ],
        ),
    );

    Ok(DataFrame::new(columns)?)
}

fn create_timeseries_data() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut columns = BTreeMap::new();

    // Timestamps (daily data for 2 weeks)
    let base_timestamp = 1678886400; // March 15, 2023
    let timestamps: Vec<Option<i64>> = (0..14)
        .map(|i| Some(base_timestamp + i * 86400)) // Add 1 day (86400 seconds)
        .collect();

    columns.insert(
        "timestamp".to_string(),
        Series::new_datetime("timestamp", timestamps),
    );

    // Stock price (with trend and volatility)
    columns.insert(
        "price".to_string(),
        Series::new_f64(
            "price",
            vec![
                Some(100.0),
                Some(102.5),
                Some(101.8),
                Some(105.2),
                Some(103.7),
                Some(107.1),
                Some(109.3),
                Some(108.6),
                Some(112.4),
                Some(110.9),
                Some(115.2),
                Some(113.8),
                Some(117.5),
                Some(119.2),
            ],
        ),
    );

    // Volume
    columns.insert(
        "volume".to_string(),
        Series::new_i32(
            "volume",
            vec![
                Some(1000000),
                Some(1200000),
                Some(900000),
                Some(1500000),
                Some(1100000),
                Some(1300000),
                Some(1600000),
                Some(1400000),
                Some(1800000),
                Some(1250000),
                Some(1700000),
                Some(1350000),
                Some(2000000),
                Some(1900000),
            ],
        ),
    );

    Ok(DataFrame::new(columns)?)
}

fn ranking_functions_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Ranking Functions");
    println!("-------------------");

    #[cfg(feature = "window_functions")]
    {
        println!("✓ Ranking functions would be implemented here");
        // Ranking function implementations would go here
    }

    #[cfg(not(feature = "window_functions"))]
    {
        println!("✗ Window functions feature not enabled - ranking functions not available");
    }

    Ok(())
}

fn offset_functions_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Offset Functions (Lag/Lead)");
    println!("------------------------------");

    #[cfg(feature = "window_functions")]
    {
        println!("✓ Offset functions would be implemented here");
        // Offset function implementations would go here
    }

    #[cfg(not(feature = "window_functions"))]
    {
        println!("✗ Window functions feature not enabled - offset functions not available");
    }

    Ok(())
}

fn aggregate_window_functions_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Aggregate Window Functions");
    println!("----------------------------");

    #[cfg(feature = "window_functions")]
    {
        println!("✓ Aggregate window functions would be implemented here");
        // Aggregate window function implementations would go here
    }

    #[cfg(not(feature = "window_functions"))]
    {
        println!(
            "✗ Window functions feature not enabled - aggregate window functions not available"
        );
    }

    Ok(())
}

fn time_based_window_functions_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. Time-based Window Functions");
    println!("-----------------------------");

    #[cfg(feature = "window_functions")]
    {
        println!("✓ Time-based window functions would be implemented here");
        // Time-based window function implementations would go here
    }

    #[cfg(not(feature = "window_functions"))]
    {
        println!(
            "✗ Window functions feature not enabled - time-based window functions not available"
        );
    }

    Ok(())
}

fn advanced_analytics_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. Advanced Analytics");
    println!("--------------------");

    #[cfg(feature = "window_functions")]
    {
        println!("✓ Advanced analytics would be implemented here");
        // Advanced analytics implementations would go here
    }

    #[cfg(not(feature = "window_functions"))]
    {
        println!("✗ Window functions feature not enabled - advanced analytics not available");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sales_data_creation() {
        let df = create_sales_data().unwrap();
        assert_eq!(df.row_count(), 12);
        assert_eq!(df.column_count(), 5);
        assert!(df.column_names().contains(&&"sales_rep".to_string()));
        assert!(df.column_names().contains(&&"region".to_string()));
        assert!(df.column_names().contains(&&"quarter".to_string()));
        assert!(df.column_names().contains(&&"sales".to_string()));
        assert!(df.column_names().contains(&&"units".to_string()));
    }

    #[test]
    fn test_timeseries_data_creation() {
        let df = create_timeseries_data().unwrap();
        assert_eq!(df.row_count(), 14);
        assert_eq!(df.column_count(), 3);
        assert!(df.column_names().contains(&&"timestamp".to_string()));
        assert!(df.column_names().contains(&&"price".to_string()));
        assert!(df.column_names().contains(&&"volume".to_string()));
    }
}
