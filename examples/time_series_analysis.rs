use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Velox Time Series Analysis Example ===\n");

    // Create sample stock price data
    let dates = vec![
        "2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04", "2024-01-05",
        "2024-01-08", "2024-01-09", "2024-01-10", "2024-01-11", "2024-01-12",
    ];
    
    let prices = vec![
        Some(100.0), Some(102.5), Some(98.0), Some(105.0), Some(107.5),
        Some(103.0), Some(109.0), Some(111.5), Some(108.0), Some(115.0),
    ];
    
    let volumes = vec![
        Some(1000), Some(1200), Some(1500), Some(900), Some(800),
        Some(1100), Some(750), Some(950), Some(1300), Some(600),
    ];

    // Create DataFrame
    let mut columns = BTreeMap::new();
    columns.insert("date".to_string(), Series::new_string("date", 
        dates.into_iter().map(|d| Some(d.to_string())).collect()));
    columns.insert("price".to_string(), Series::new_f64("price", prices));
    columns.insert("volume".to_string(), Series::new_i32("volume", volumes));

    let df = DataFrame::new(columns)?;
    
    println!("Original Stock Data:");
    println!("{}\n", df);

    // Calculate rolling statistics
    println!("=== Rolling Window Analysis (3-day window) ===");
    let df_with_rolling = df
        .rolling_mean(vec!["price".to_string(), "volume".to_string()], 3)?
        .rolling_std(vec!["price".to_string()], 3)?;
    
    println!("Data with 3-day Rolling Mean and Standard Deviation:");
    println!("{}\n", df_with_rolling);

    // Calculate percentage changes
    println!("=== Percentage Change Analysis ===");
    let df_with_pct_change = df.pct_change(vec!["price".to_string()])?;
    
    println!("Data with Price Percentage Changes:");
    println!("{}\n", df_with_pct_change);

    // Calculate cumulative sum
    println!("=== Cumulative Analysis ===");
    let df_with_cumsum = df.cumsum(vec!["volume".to_string()])?;
    
    println!("Data with Cumulative Volume:");
    println!("{}\n", df_with_cumsum);

    // Demonstrate Series-level operations
    println!("=== Series-Level Time Series Operations ===");
    let price_series = df.get_column("price").unwrap();
    
    // Rolling operations on individual series
    let rolling_mean = price_series.rolling_mean(3)?;
    let rolling_min = price_series.rolling_min(3)?;
    let rolling_max = price_series.rolling_max(3)?;
    let pct_change = price_series.pct_change()?;
    
    println!("Price Series Analysis:");
    println!("Original Prices: {:?}", price_series);
    println!("3-day Rolling Mean: {:?}", rolling_mean);
    println!("3-day Rolling Min: {:?}", rolling_min);
    println!("3-day Rolling Max: {:?}", rolling_max);
    println!("Percentage Change: {:?}", pct_change);

    // Advanced analysis: Combine multiple rolling windows
    println!("\n=== Advanced Multi-Window Analysis ===");
    let df_advanced = df
        .rolling_mean(vec!["price".to_string()], 3)?
        .rolling_mean(vec!["price".to_string()], 5)?
        .rolling_std(vec!["price".to_string()], 3)?;
    
    println!("Data with Multiple Rolling Windows (3-day and 5-day means, 3-day std):");
    println!("{}", df_advanced);

    Ok(())
}