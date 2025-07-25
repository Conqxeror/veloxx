use veloxx::distributed::DistributedDataFrame;
use veloxx::dataframe::DataFrame;
use veloxx::series::Series;
use std::collections::BTreeMap;

#[test]
fn test_distributed_dataframe_creation() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3), Some(4)]));
    columns.insert("value".to_string(), Series::new_f64("value", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
    assert_eq!(distributed_df.partition_count(), 2);
}

#[test]
fn test_distributed_dataframe_single_partition() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 1).unwrap();
    assert_eq!(distributed_df.partition_count(), 1);
}

#[test]
fn test_distributed_dataframe_empty() {
    let columns = BTreeMap::new();
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
    assert_eq!(distributed_df.partition_count(), 1); // Empty DF gets 1 partition
}

#[test]
fn test_distributed_dataframe_zero_partitions() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1)]));
    let df = DataFrame::new(columns).unwrap();
    
    let result = DistributedDataFrame::from_dataframe(df, 0);
    assert!(result.is_err());
}

#[test]
fn test_distributed_dataframe_collect() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2), Some(3)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df.clone(), 2).unwrap();
    let collected_df = distributed_df.collect().unwrap();
    
    assert_eq!(collected_df.row_count(), df.row_count());
    assert_eq!(collected_df.column_count(), df.column_count());
}

#[test]
fn test_distributed_dataframe_more_partitions_than_rows() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 5).unwrap();
    // Should create only as many partitions as there are rows
    assert!(distributed_df.partition_count() <= 2);
}

#[test]
fn test_distributed_dataframe_large_dataset() {
    let mut columns = BTreeMap::new();
    let data: Vec<Option<i32>> = (1..=1000).map(Some).collect();
    columns.insert("id".to_string(), Series::new_i32("id", data));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 10).unwrap();
    assert_eq!(distributed_df.partition_count(), 10);
    
    let collected_df = distributed_df.collect().unwrap();
    assert_eq!(collected_df.row_count(), 1000);
}

#[test]
fn test_distributed_dataframe_clone() {
    let mut columns = BTreeMap::new();
    columns.insert("id".to_string(), Series::new_i32("id", vec![Some(1), Some(2)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 2).unwrap();
    let cloned_df = distributed_df.clone();
    
    assert_eq!(distributed_df.partition_count(), cloned_df.partition_count());
}

#[test]
fn test_distributed_dataframe_debug() {
    let mut columns = BTreeMap::new();
    columns.insert("test".to_string(), Series::new_i32("test", vec![Some(1)]));
    let df = DataFrame::new(columns).unwrap();
    
    let distributed_df = DistributedDataFrame::from_dataframe(df, 1).unwrap();
    let debug_str = format!("{:?}", distributed_df);
    
    assert!(debug_str.contains("DistributedDataFrame"));
}