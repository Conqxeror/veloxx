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
use arrow::csv::reader::ReaderBuilder;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow::datatypes::{DataType as ArrowDataType, Field, Schema, TimeUnit};
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow::record_batch::RecordBatch;
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use indexmap::IndexMap;
#[cfg(all(
    feature = "advanced_io",
    feature = "arrow",
    not(target_arch = "wasm32")
))]
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
#[cfg(all(
    feature = "advanced_io",
    feature = "arrow",
    not(target_arch = "wasm32")
))]
use parquet::arrow::arrow_writer::ArrowWriter;
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
        return Ok(DataFrame::new(IndexMap::new()));
    }

    let schema = record_batches[0].schema();
    let mut columns: IndexMap<String, Series> = IndexMap::new();

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

    Ok(DataFrame::new(columns))
}

#[cfg(all(
    feature = "advanced_io",
    feature = "arrow",
    not(target_arch = "wasm32")
))]
pub fn read_parquet_to_dataframe(file_path: &str) -> Result<DataFrame, VeloxxError> {
    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;

    let mut record_batches: Vec<RecordBatch> = Vec::new();
    for batch in reader {
        record_batches.push(batch?);
    }

    if record_batches.is_empty() {
        return Ok(DataFrame::new(indexmap::IndexMap::new()));
    }

    let schema = record_batches[0].schema();
    let mut columns: indexmap::IndexMap<String, Series> = indexmap::IndexMap::new();

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

    Ok(DataFrame::new(columns))
}

#[cfg(all(
    feature = "advanced_io",
    feature = "arrow",
    not(target_arch = "wasm32")
))]
pub fn write_parquet_from_dataframe(
    dataframe: &DataFrame,
    file_path: &str,
) -> Result<(), VeloxxError> {
    let file = File::create(file_path)?;

    // Convert DataFrame schema to Arrow Schema
    let mut fields = Vec::new();
    let column_names = dataframe.column_names();
    for name in &column_names {
        let series = dataframe.get_column(name).unwrap();
        let arrow_type = match series.data_type() {
            crate::types::DataType::I32 => ArrowDataType::Int32,
            crate::types::DataType::F64 => ArrowDataType::Float64,
            crate::types::DataType::Bool => ArrowDataType::Boolean,
            crate::types::DataType::String => ArrowDataType::Utf8,
            crate::types::DataType::DateTime => {
                ArrowDataType::Timestamp(TimeUnit::Nanosecond, None)
            }
        };
        fields.push(Field::new(name.as_str(), arrow_type, true));
    }
    let schema = Arc::new(Schema::new(fields));

    // Create RecordBatch
    let mut columns = Vec::new();
    for name in column_names {
        let series = dataframe.get_column(&name).unwrap();
        columns.push(series.to_arrow_array());
    }
    let batch = RecordBatch::try_new(schema.clone(), columns)?;

    // Write to Parquet
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}
