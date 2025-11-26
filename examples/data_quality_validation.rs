//! Example demonstrating data quality and validation features in Velox
//!
//! This example shows the structure for data quality operations.
//! The actual implementations are placeholders until the data_quality feature is fully implemented.

use indexmap::IndexMap;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Velox Data Quality & Validation Example");
    println!("=======================================");

    // Create sample data with various quality issues
    let sample_df = create_sample_data_with_issues()?;
    println!("Sample data with quality issues:");
    println!("{}", sample_df);

    // Demonstrate schema validation
    schema_validation_example(&sample_df)?;

    // Demonstrate data profiling
    data_profiling_example(&sample_df)?;

    // Demonstrate anomaly detection
    anomaly_detection_example(&sample_df)?;

    // Demonstrate data validation
    data_validation_example(&sample_df)?;

    // Demonstrate quality reporting
    quality_reporting_example(&sample_df)?;

    println!("\nData quality and validation examples completed!");
    println!("Note: Enable the 'data_quality' feature to use actual validation operations:");
    println!("cargo run --example data_quality_validation --features data_quality");

    Ok(())
}

fn create_sample_data_with_issues() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut columns = IndexMap::new();

    // Employee ID column
    columns.insert(
        "employee_id".to_string(),
        Series::new_i32(
            "employee_id",
            vec![
                Some(1001),
                Some(1002),
                Some(1003),
                Some(1004),
                Some(1005),
                Some(1006),
                Some(1007),
                Some(1008),
                Some(1009),
                Some(1010),
            ],
        ),
    );

    // Name column (some missing values)
    columns.insert(
        "name".to_string(),
        Series::new_string(
            "name",
            vec![
                Some("Alice Johnson".to_string()),
                Some("Bob Smith".to_string()),
                None, // Missing name
                Some("Diana Prince".to_string()),
                Some("Eve Adams".to_string()),
                Some("Frank Miller".to_string()),
                Some("Grace Lee".to_string()),
                Some("Henry Davis".to_string()),
                Some("Ivy Chen".to_string()),
                Some("Jack Wilson".to_string()),
            ],
        ),
    );

    // Age column (with outliers)
    columns.insert(
        "age".to_string(),
        Series::new_i32(
            "age",
            vec![
                Some(28),
                Some(34),
                Some(29),
                Some(45),
                Some(31),
                Some(38),
                Some(150), // Outlier - impossible age
                Some(27),
                Some(33),
                Some(-5), // Outlier - negative age
            ],
        ),
    );

    // Email column (with invalid formats)
    columns.insert(
        "email".to_string(),
        Series::new_string(
            "email",
            vec![
                Some("alice.johnson@company.com".to_string()),
                Some("bob.smith@company.com".to_string()),
                Some("charlie.invalid-email".to_string()), // Invalid format
                Some("diana.prince@company.com".to_string()),
                Some("eve.adams@company.com".to_string()),
                Some("frank.miller@company.com".to_string()),
                Some("grace.lee@".to_string()), // Invalid format
                Some("henry.davis@company.com".to_string()),
                Some("ivy.chen@company.com".to_string()),
                None, // Missing email
            ],
        ),
    );

    // Salary column (with outliers)
    columns.insert(
        "salary".to_string(),
        Series::new_f64(
            "salary",
            vec![
                Some(65000.0),
                Some(72000.0),
                Some(58000.0),
                Some(95000.0),
                Some(68000.0),
                Some(75000.0),
                Some(1000000.0), // Outlier - very high salary
                Some(62000.0),
                Some(70000.0),
                Some(0.0), // Outlier - zero salary
            ],
        ),
    );

    // Department column
    columns.insert(
        "department".to_string(),
        Series::new_string(
            "department",
            vec![
                Some("Engineering".to_string()),
                Some("Marketing".to_string()),
                Some("Engineering".to_string()),
                Some("Sales".to_string()),
                Some("Engineering".to_string()),
                Some("Marketing".to_string()),
                Some("HR".to_string()),
                Some("Engineering".to_string()),
                Some("Sales".to_string()),
                Some("Finance".to_string()),
            ],
        ),
    );

    Ok(DataFrame::new(columns))
}

fn schema_validation_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. Schema Validation");
    println!("-------------------");

    #[cfg(feature = "data_quality")]
    {
        println!("✓ Schema validation would be implemented here");
        // Schema validation implementation would go here
    }

    #[cfg(not(feature = "data_quality"))]
    {
        println!("✗ Data quality feature not enabled - schema validation not available");
    }

    Ok(())
}

fn data_profiling_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. Data Profiling");
    println!("----------------");

    #[cfg(feature = "data_quality")]
    {
        println!("✓ Data profiling would be implemented here");
        // Data profiling implementation would go here
    }

    #[cfg(not(feature = "data_quality"))]
    {
        println!("✗ Data quality feature not enabled - data profiling not available");
    }

    Ok(())
}

fn anomaly_detection_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. Anomaly Detection");
    println!("-------------------");

    #[cfg(feature = "data_quality")]
    {
        println!("✓ Anomaly detection would be implemented here");
        // Anomaly detection implementation would go here
    }

    #[cfg(not(feature = "data_quality"))]
    {
        println!("✗ Data quality feature not enabled - anomaly detection not available");
    }

    Ok(())
}

fn data_validation_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. Data Validation");
    println!("-----------------");

    #[cfg(feature = "data_quality")]
    {
        println!("✓ Data validation would be implemented here");
        // Data validation implementation would go here
    }

    #[cfg(not(feature = "data_quality"))]
    {
        println!("✗ Data quality feature not enabled - data validation not available");
    }

    Ok(())
}

fn quality_reporting_example(_df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. Quality Reporting");
    println!("-------------------");

    #[cfg(feature = "data_quality")]
    {
        println!("✓ Quality reporting would be implemented here");
        // Quality reporting implementation would go here
    }

    #[cfg(not(feature = "data_quality"))]
    {
        println!("✗ Data quality feature not enabled - quality reporting not available");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data_creation() {
        let df = create_sample_data_with_issues().unwrap();
        assert_eq!(df.row_count(), 10);
        assert_eq!(df.column_count(), 6);
        assert!(df.column_names().contains(&&"employee_id".to_string()));
        assert!(df.column_names().contains(&&"name".to_string()));
        assert!(df.column_names().contains(&&"age".to_string()));
        assert!(df.column_names().contains(&&"email".to_string()));
        assert!(df.column_names().contains(&&"salary".to_string()));
        assert!(df.column_names().contains(&&"department".to_string()));
    }
}
