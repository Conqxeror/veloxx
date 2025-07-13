//! Example demonstrating data visualization capabilities in Velox
//!
//! This example shows how to create various types of plots from DataFrame data,
//! including line plots, scatter plots, and bar charts.

use std::collections::BTreeMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

#[cfg(feature = "visualization")]
use veloxx::visualization::{ChartType, Plot, PlotConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Velox Data Visualization Example");
    println!("================================");

    // Create sample data for line plot
    create_line_plot_example()?;

    // Create sample data for scatter plot
    create_scatter_plot_example()?;

    // Create sample data for bar chart
    create_bar_chart_example()?;

    println!("\nVisualization examples completed!");
    println!("Note: Enable the 'visualization' feature to generate actual plots:");
    println!("cargo run --example data_visualization --features visualization");

    Ok(())
}

fn create_line_plot_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Line Plot Example");
    println!("-------------------");

    // Create time series data
    let mut columns = BTreeMap::new();
    columns.insert(
        "time".to_string(),
        Series::new_f64(
            "time",
            vec![
                Some(0.0),
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
            ],
        ),
    );
    columns.insert(
        "value".to_string(),
        Series::new_f64(
            "value",
            vec![
                Some(0.0),
                Some(1.0),
                Some(4.0),
                Some(9.0),
                Some(16.0),
                Some(25.0),
                Some(36.0),
                Some(49.0),
                Some(64.0),
                Some(81.0),
            ],
        ),
    );

    let df = DataFrame::new(columns)?;
    println!("Created DataFrame with time series data:");
    println!("{}", df);

    #[cfg(feature = "visualization")]
    {
        let config = PlotConfig {
            title: "Time Series - Quadratic Growth".to_string(),
            x_label: "Time".to_string(),
            y_label: "Value".to_string(),
            ..Default::default()
        };

        let plot = Plot::new(&df, ChartType::Line)
            .with_config(config)
            .with_columns("time", "value");

        plot.save("line_plot_example.svg")?;
        println!("✓ Line plot saved as 'line_plot_example.svg'");
    }

    #[cfg(not(feature = "visualization"))]
    {
        println!("✗ Visualization feature not enabled - plot not generated");
    }

    Ok(())
}

fn create_scatter_plot_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Scatter Plot Example");
    println!("----------------------");

    // Create correlation data
    let mut columns = BTreeMap::new();
    columns.insert(
        "x".to_string(),
        Series::new_f64(
            "x",
            vec![
                Some(1.0),
                Some(2.0),
                Some(3.0),
                Some(4.0),
                Some(5.0),
                Some(6.0),
                Some(7.0),
                Some(8.0),
                Some(9.0),
                Some(10.0),
            ],
        ),
    );
    columns.insert(
        "y".to_string(),
        Series::new_f64(
            "y",
            vec![
                Some(2.1),
                Some(3.9),
                Some(6.2),
                Some(7.8),
                Some(10.1),
                Some(11.9),
                Some(14.2),
                Some(15.8),
                Some(18.1),
                Some(19.9),
            ],
        ),
    );

    let df = DataFrame::new(columns)?;
    println!("Created DataFrame with correlation data:");
    println!("{}", df);

    #[cfg(feature = "visualization")]
    {
        let config = PlotConfig {
            title: "Scatter Plot - Linear Correlation".to_string(),
            x_label: "X Values".to_string(),
            y_label: "Y Values".to_string(),
            ..Default::default()
        };

        let plot = Plot::new(&df, ChartType::Scatter)
            .with_config(config)
            .with_columns("x", "y");

        plot.save("scatter_plot_example.svg")?;
        println!("✓ Scatter plot saved as 'scatter_plot_example.svg'");
    }

    #[cfg(not(feature = "visualization"))]
    {
        println!("✗ Visualization feature not enabled - plot not generated");
    }

    Ok(())
}

fn create_bar_chart_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Bar Chart Example");
    println!("-------------------");

    // Create categorical data
    let mut columns = BTreeMap::new();
    columns.insert(
        "category".to_string(),
        Series::new_string(
            "category",
            vec![
                Some("Product A".to_string()),
                Some("Product B".to_string()),
                Some("Product C".to_string()),
                Some("Product D".to_string()),
                Some("Product E".to_string()),
            ],
        ),
    );
    columns.insert(
        "sales".to_string(),
        Series::new_f64(
            "sales",
            vec![
                Some(150.0),
                Some(230.0),
                Some(180.0),
                Some(320.0),
                Some(210.0),
            ],
        ),
    );

    let df = DataFrame::new(columns)?;
    println!("Created DataFrame with sales data:");
    println!("{}", df);

    #[cfg(feature = "visualization")]
    {
        let config = PlotConfig {
            title: "Sales by Product Category".to_string(),
            x_label: "Product".to_string(),
            y_label: "Sales ($000)".to_string(),
            ..Default::default()
        };

        let plot = Plot::new(&df, ChartType::Bar)
            .with_config(config)
            .with_columns("category", "sales");

        plot.save("bar_chart_example.svg")?;
        println!("✓ Bar chart saved as 'bar_chart_example.svg'");
    }

    #[cfg(not(feature = "visualization"))]
    {
        println!("✗ Visualization feature not enabled - plot not generated");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data_creation() {
        // Test that we can create the example data without errors
        assert!(create_line_plot_example().is_ok());
        assert!(create_scatter_plot_example().is_ok());
        assert!(create_bar_chart_example().is_ok());
    }
}
