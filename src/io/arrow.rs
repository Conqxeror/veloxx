#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use crate::dataframe::DataFrame;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use crate::series::Series;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use crate::VeloxxError;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "arrow")]
// Arrow-specific code starts here
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow::csv::reader::Format;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow::record_batch::RecordBatch;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow_csv::ReaderBuilder;
#[cfg(all(
    feature = "advanced_io",
    feature = "arrow",
    not(target_arch = "wasm32")
))]
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use std::collections::HashMap;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use std::fs::File;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use std::io::BufReader;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use std::sync::Arc;

#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
pub fn read_csv_to_dataframe(file_path: &str) -> Result<DataFrame, VeloxxError> {
    let file = File::open(file_path)?;
    let format = Format::default().with_header(true).with_delimiter(b',');
    let (schema, _num_records) = format.infer_schema(file.try_clone()?, Some(100))?;

    let builder = ReaderBuilder::new(Arc::new(schema)).with_header(true);
    let mut reader = builder.build(BufReader::new(file))?;

    let mut record_batches: Vec<RecordBatch> = Vec::new();
    while let Some(batch) = reader.next().transpose()? {
        record_batches.push(batch);
    }

    if record_batches.is_empty() {
        return DataFrame::new(HashMap::new());
    }

    let schema = record_batches[0].schema();
    let mut columns: HashMap<String, Series> = HashMap::new();

    for i in 0..schema.fields().len() {
        let field = schema.field(i);
        let mut series_data: Vec<Series> = Vec::new();
        for batch in &record_batches {
            let array = batch.column(i);
            series_data.push(Series::from_arrow_array(
                array.clone(),
                field.name().clone(),
            )?);
        }
        columns.insert(field.name().clone(), Series::concat(series_data)?);
    }

    DataFrame::new(columns)
}

#[cfg(feature = "advanced_io")]
pub fn read_parquet_to_dataframe(file_path: &str) -> Result<DataFrame, VeloxxError> {
    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    let mut record_batches: Vec<RecordBatch> = Vec::new();
    for batch in reader {
        record_batches.push(batch?);
    }

    if record_batches.is_empty() {
        return DataFrame::new(std::collections::HashMap::new());
    }

    let schema = record_batches[0].schema();
    let mut columns: std::collections::HashMap<String, Series> = std::collections::HashMap::new();

    for i in 0..schema.fields().len() {
        let field = schema.field(i);
        let mut series_data: Vec<Series> = Vec::new();
        for batch in &record_batches {
            let array = batch.column(i);
            series_data.push(Series::from_arrow_array(
                array.clone(),
                field.name().clone(),
            )?);
        }
        columns.insert(field.name().clone(), Series::concat(series_data)?);
    }

    DataFrame::new(columns)
}
