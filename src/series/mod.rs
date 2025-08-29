use crate::types::{DataType, Value};
use crate::VeloxxError;

// Arrow imports only for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
use arrow::array::{
    ArrayRef, BooleanArray, Float64Array, Int32Array, StringArray, TimestampNanosecondArray,
};
#[cfg(not(target_arch = "wasm32"))]
use arrow::datatypes::{DataType as ArrowDataType, TimeUnit};

// SIMD trait imports - only for native targets
// Note: we use concrete traits in method scopes to minimize compile-time coupling



#[derive(Debug, PartialEq, Clone)]
pub enum Series {
    I32(String, Vec<i32>, Vec<bool>),
    F64(String, Vec<f64>, Vec<bool>),
    Bool(String, Vec<bool>, Vec<bool>),
    String(String, Vec<String>, Vec<bool>),
    DateTime(String, Vec<i64>, Vec<bool>),
}

impl Series {
    pub fn name(&self) -> &str {
        match self {
            Series::I32(name, _, _) => name,
            Series::F64(name, _, _) => name,
            Series::Bool(name, _, _) => name,
            Series::String(name, _, _) => name,
            Series::DateTime(name, _, _) => name,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Series::I32(_, values, _) => values.len(),
            Series::F64(_, values, _) => values.len(),
            Series::Bool(_, values, _) => values.len(),
            Series::String(_, values, _) => values.len(),
            Series::DateTime(_, values, _) => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data_type(&self) -> DataType {
        match self {
            Series::I32(_, _, _) => DataType::I32,
            Series::F64(_, _, _) => DataType::F64,
            Series::Bool(_, _, _) => DataType::Bool,
            Series::String(_, _, _) => DataType::String,
            Series::DateTime(_, _, _) => DataType::DateTime,
        }
    }

    pub fn get_value(&self, index: usize) -> Option<Value> {
        match self {
            Series::I32(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(Value::I32(values[index]))
                } else {
                    None
                }
            }
            Series::F64(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(Value::F64(values[index]))
                } else {
                    None
                }
            }
            Series::Bool(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(Value::Bool(values[index]))
                } else {
                    None
                }
            }
            Series::String(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(Value::String(values[index].clone()))
                } else {
                    None
                }
            }
            Series::DateTime(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(Value::DateTime(values[index]))
                } else {
                    None
                }
            }
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Series::I32(_, _, _) | Series::F64(_, _, _))
    }
    /// Returns numeric value as f64 at index if present and valid, else None
    fn get_numeric_f64(&self, index: usize) -> Option<f64> {
        match self {
            Series::F64(_, values, validity) => {
                if index < values.len() && validity[index] { Some(values[index]) } else { None }
            }
            Series::I32(_, values, validity) => {
                if index < values.len() && validity[index] { Some(values[index] as f64) } else { None }
            }
            _ => None,
        }
    }
    /// Returns i32 value at index if present and valid, else None
    pub fn get_i32(&self, index: usize) -> Option<i32> {
        match self {
            Series::I32(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(values[index])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Returns f64 value at index if present and valid, else None
    pub fn get_f64(&self, index: usize) -> Option<f64> {
        match self {
            Series::F64(_, values, validity) => {
                if index < values.len() && validity[index] {
                    Some(values[index])
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Compute the percentile for a given value (0.0 to 100.0) using parallel sorting.
    pub fn percentile(&self, pct: f64) -> Result<Option<Value>, VeloxxError> {
        if !(0.0..=100.0).contains(&pct) {
            return Err(VeloxxError::InvalidOperation(
                "Percentile must be between 0.0 and 100.0".to_string(),
            ));
        }
        let prob = pct / 100.0;
        match self {
            Series::I32(_, values, bitmap) => {
                use rayon::prelude::*;
                let mut non_null_data: Vec<i32> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.par_sort_unstable();
                let n = non_null_data.len();
                let pos = ((n - 1) as f64 * prob).round() as usize;
                Ok(Some(Value::I32(non_null_data[pos])))
            }
            Series::F64(_, values, bitmap) => {
                use rayon::prelude::*;
                let mut non_null_data: Vec<f64> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data
                    .par_sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let n = non_null_data.len();
                let pos = ((n - 1) as f64 * prob).round() as usize;
                Ok(Some(Value::F64(non_null_data[pos])))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Percentile operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }
    /// Compute the quantile for a given probability (0.0 to 1.0) using parallel sorting.
    pub fn quantile(&self, prob: f64) -> Result<Option<Value>, VeloxxError> {
        if !(0.0..=1.0).contains(&prob) {
            return Err(VeloxxError::InvalidOperation(
                "Quantile probability must be between 0.0 and 1.0".to_string(),
            ));
        }
        match self {
            Series::I32(_, values, bitmap) => {
                use rayon::prelude::*;
                let mut non_null_data: Vec<i32> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data.par_sort_unstable();
                let n = non_null_data.len();
                let pos = ((n - 1) as f64 * prob).round() as usize;
                Ok(Some(Value::I32(non_null_data[pos])))
            }
            Series::F64(_, values, bitmap) => {
                use rayon::prelude::*;
                let mut non_null_data: Vec<f64> = values
                    .par_iter()
                    .zip(bitmap.par_iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect();
                if non_null_data.is_empty() {
                    return Ok(None);
                }
                non_null_data
                    .par_sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let n = non_null_data.len();
                let pos = ((n - 1) as f64 * prob).round() as usize;
                Ok(Some(Value::F64(non_null_data[pos])))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Quantile operation not supported for {:?} series.",
                self.data_type()
            ))),
        }
    }
    pub fn new_i32(name: &str, data: Vec<Option<i32>>) -> Self {
        let mut values = Vec::with_capacity(data.len());
        let mut bitmap = Vec::with_capacity(data.len());
        for v in data {
            match v {
                Some(val) => {
                    values.push(val);
                    bitmap.push(true);
                }
                None => {
                    values.push(0); // placeholder, ignored if bitmap is false
                    bitmap.push(false);
                }
            }
        }
        Series::I32(name.to_string(), values, bitmap)
    }

    pub fn new_f64(name: &str, data: Vec<Option<f64>>) -> Self {
        let mut values = Vec::with_capacity(data.len());
        let mut bitmap = Vec::with_capacity(data.len());
        for v in data {
            match v {
                Some(val) => {
                    values.push(val);
                    bitmap.push(true);
                }
                None => {
                    values.push(0.0); // placeholder
                    bitmap.push(false);
                }
            }
        }
        Series::F64(name.to_string(), values, bitmap)
    }

    pub fn new_bool(name: &str, data: Vec<Option<bool>>) -> Self {
        let mut values = Vec::with_capacity(data.len());
        let mut bitmap = Vec::with_capacity(data.len());
        for v in data {
            match v {
                Some(val) => {
                    values.push(val);
                    bitmap.push(true);
                }
                None => {
                    values.push(false); // placeholder
                    bitmap.push(false);
                }
            }
        }
        Series::Bool(name.to_string(), values, bitmap)
    }

    pub fn new_string(name: &str, data: Vec<Option<String>>) -> Self {
        let mut values = Vec::with_capacity(data.len());
        let mut bitmap = Vec::with_capacity(data.len());
        for v in data {
            match v {
                Some(val) => {
                    values.push(val);
                    bitmap.push(true);
                }
                None => {
                    values.push(String::new()); // placeholder
                    bitmap.push(false);
                }
            }
        }
        Series::String(name.to_string(), values, bitmap)
    }

    pub fn new_datetime(name: &str, data: Vec<Option<i64>>) -> Self {
        let mut values = Vec::with_capacity(data.len());
        let mut bitmap = Vec::with_capacity(data.len());
        for v in data {
            match v {
                Some(val) => {
                    values.push(val);
                    bitmap.push(true);
                }
                None => {
                    values.push(0); // placeholder
                    bitmap.push(false);
                }
            }
        }
        Series::DateTime(name.to_string(), values, bitmap)
    }

    /// Create a Series from an Arrow array (not available in WASM)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_arrow_array(array: ArrayRef, name: String) -> Result<Self, VeloxxError> {
        match array.data_type() {
            ArrowDataType::Int32 => {
                let arr = array.as_any().downcast_ref::<Int32Array>().ok_or_else(|| {
                    VeloxxError::Parsing("Failed to downcast to Int32Array".to_string())
                })?;
                let values: Vec<i32> = arr.iter().flatten().collect();
                let bitmap: Vec<bool> = arr.iter().map(|x| x.is_some()).collect();
                Ok(Series::I32(name, values, bitmap))
            }
            ArrowDataType::Float64 => {
                let arr = array
                    .as_any()
                    .downcast_ref::<Float64Array>()
                    .ok_or_else(|| {
                        VeloxxError::Parsing("Failed to downcast to Float64Array".to_string())
                    })?;
                let values: Vec<f64> = arr.iter().flatten().collect();
                let bitmap: Vec<bool> = arr.iter().map(|x| x.is_some()).collect();
                Ok(Series::F64(name, values, bitmap))
            }
            ArrowDataType::Boolean => {
                let arr = array
                    .as_any()
                    .downcast_ref::<BooleanArray>()
                    .ok_or_else(|| {
                        VeloxxError::Parsing("Failed to downcast to BooleanArray".to_string())
                    })?;
                let values: Vec<bool> = arr.iter().flatten().collect();
                let bitmap: Vec<bool> = arr.iter().map(|x| x.is_some()).collect();
                Ok(Series::Bool(name, values, bitmap))
            }
            ArrowDataType::Utf8 => {
                let arr = array
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .ok_or_else(|| {
                        VeloxxError::Parsing("Failed to downcast to StringArray".to_string())
                    })?;
                let values: Vec<String> = arr.iter().flatten().map(|s| s.to_string()).collect();
                let bitmap: Vec<bool> = arr.iter().map(|x| x.is_some()).collect();
                Ok(Series::String(name, values, bitmap))
            }
            ArrowDataType::Timestamp(TimeUnit::Nanosecond, _) => {
                let arr = array
                    .as_any()
                    .downcast_ref::<TimestampNanosecondArray>()
                    .ok_or_else(|| {
                        VeloxxError::Parsing(
                            "Failed to downcast to TimestampNanosecondArray".to_string(),
                        )
                    })?;
                let values: Vec<i64> = arr.iter().flatten().collect();
                let bitmap: Vec<bool> = arr.iter().map(|x| x.is_some()).collect();
                Ok(Series::DateTime(name, values, bitmap))
            }
            _ => Err(VeloxxError::Unsupported(format!(
                "Unsupported Arrow data type: {:?}",
                array.data_type()
            ))),
        }
    }

    pub fn concat(series_list: Vec<Series>) -> Result<Self, VeloxxError> {
        if series_list.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "Cannot concatenate empty list of Series".to_string(),
            ));
        }

        let first_series = &series_list[0];
        let name = first_series.name().to_string();
        let data_type = first_series.data_type();

        for s in &series_list {
            if s.data_type() != data_type {
                return Err(VeloxxError::DataTypeMismatch(
                    "Cannot concatenate Series of different types".to_string(),
                ));
            }
        }

        match data_type {
            DataType::I32 => {
                let mut values = Vec::new();
                let mut bitmap = Vec::new();
                for s in series_list {
                    if let Series::I32(_, v, b) = s {
                        values.extend(v);
                        bitmap.extend(b);
                    } else {
                        unreachable!(); // Should be caught by type check above
                    }
                }
                Ok(Series::I32(name, values, bitmap))
            }
            DataType::F64 => {
                let mut values = Vec::new();
                let mut bitmap = Vec::new();
                for s in series_list {
                    if let Series::F64(_, v, b) = s {
                        values.extend(v);
                        bitmap.extend(b);
                    } else {
                        unreachable!();
                    }
                }
                Ok(Series::F64(name, values, bitmap))
            }
            DataType::Bool => {
                let mut values = Vec::new();
                let mut bitmap = Vec::new();
                for s in series_list {
                    if let Series::Bool(_, v, b) = s {
                        values.extend(v);
                        bitmap.extend(b);
                    } else {
                        unreachable!();
                    }
                }
                Ok(Series::Bool(name, values, bitmap))
            }
            DataType::String => {
                let mut values = Vec::new();
                let mut bitmap = Vec::new();
                for s in series_list {
                    if let Series::String(_, v, b) = s {
                        values.extend(v);
                        bitmap.extend(b);
                    } else {
                        unreachable!();
                    }
                }
                Ok(Series::String(name, values, bitmap))
            }
            DataType::DateTime => {
                let mut values = Vec::new();
                let mut bitmap = Vec::new();
                for s in series_list {
                    if let Series::DateTime(_, v, b) = s {
                        values.extend(v);
                        bitmap.extend(b);
                    } else {
                        unreachable!();
                    }
                }
                Ok(Series::DateTime(name, values, bitmap))
            }
        }
    }

    pub fn value_counts(&self) -> Result<Series, VeloxxError> {
        let name = format!("{}_value_counts", self.name());
        match self {
            Series::I32(_, values, bitmap) => {
                let mut counts = std::collections::HashMap::new();
                for (&v, &b) in values.iter().zip(bitmap.iter()) {
                    if b {
                        *counts.entry(v).or_insert(0usize) += 1;
                    }
                }
                let mut unique_values = Vec::with_capacity(counts.len());
                let mut unique_counts = Vec::with_capacity(counts.len());
                let mut unique_bitmap = Vec::with_capacity(counts.len());
                for (val, count) in counts {
                    unique_values.push(val);
                    unique_counts.push(count as i32);
                    unique_bitmap.push(true);
                }
                // Return as a two-column Series: values and counts (as I32)
                // Here, we return a Series::I32 with values being the counts, and the name indicating value_counts
                // For a more complex return, consider a DataFrame or tuple
                Ok(Series::I32(name, unique_counts, unique_bitmap))
            }
            Series::F64(_, values, bitmap) => {
                let mut counts = std::collections::HashMap::new();
                for (&v, &b) in values.iter().zip(bitmap.iter()) {
                    if b {
                        *counts.entry(v.to_bits()).or_insert(0usize) += 1;
                    }
                }
                let mut unique_values = Vec::with_capacity(counts.len());
                let mut unique_counts = Vec::with_capacity(counts.len());
                let mut unique_bitmap = Vec::with_capacity(counts.len());
                for (bits, count) in counts {
                    unique_values.push(f64::from_bits(bits));
                    unique_counts.push(count as i32);
                    unique_bitmap.push(true);
                }
                // Return as a two-column Series: values and counts (as I32)
                // Here, we return a Series::I32 with values being the counts, and the name indicating value_counts
                // For a more complex return, consider a DataFrame or tuple
                Ok(Series::I32(name, unique_counts, unique_bitmap))
            }
            _ => Err(VeloxxError::Unsupported(
                "value_counts only supported for I32 and F64 series".to_string(),
            )),
        }
    }
    /// Interpolates null values using linear interpolation for numeric series.
    ///
    /// This method performs linear interpolation on null values. It only works
    /// on numeric series (I32 and F64). Null values at the beginning or end
    /// of the series remain as null.
    ///
    /// # Returns
    ///
    /// A `Result` which is `Ok(Series)` containing a new series with interpolated values,
    /// or `Err(VeloxxError)` if interpolation is not supported for this series type.
    pub fn interpolate_nulls(&self) -> Result<Self, VeloxxError> {
        if !self.is_numeric() {
            return Err(VeloxxError::Unsupported(
                "Interpolation only supported for numeric series".to_string(),
            ));
        }

        let name = self.name().to_string();
        let (values, bitmap): (Vec<Option<f64>>, Vec<bool>) = match self {
            Series::I32(_, values, bitmap) => values
                .iter()
                .zip(bitmap.iter())
                .map(|(&v, &b)| (if b { Some(v as f64) } else { None }, b))
                .unzip(),
            Series::F64(_, values, bitmap) => values
                .iter()
                .zip(bitmap.iter())
                .map(|(&v, &b)| (if b { Some(v) } else { None }, b))
                .unzip(),
            _ => unreachable!(),
        };

        let mut interpolated_values = vec![0.0; values.len()]; // Initialize with placeholders
        let mut interpolated_bitmap = vec![false; values.len()]; // Initialize with false

        let mut i = 0;
        // Skip leading nulls - they remain as null
        while i < values.len() && !bitmap[i] {
            i += 1;
        }

        let mut last_non_null_idx = None;
        for j in i..values.len() {
            if bitmap[j] {
                // This is a non-null value
                if let Some(last_idx) = last_non_null_idx {
                    // We have a previous non-null value, interpolate between them
                    let start_val_unwrapped: f64 = match &values[last_idx] {
                        Some(val) => *val,
                        None => unreachable!(), // We know this is Some because of the bitmap check
                    };
                    let end_val_unwrapped: f64 = match &values[j] {
                        Some(val) => *val,
                        None => unreachable!(), // We know this is Some because of the bitmap check
                    };
                    let num_steps = (j - last_idx) as f64;

                    // Interpolate values between last_idx and j
                    for k in (last_idx + 1)..j {
                        let interpolated_val = start_val_unwrapped
                            + (end_val_unwrapped - start_val_unwrapped)
                                * ((k - last_idx) as f64 / num_steps);
                        interpolated_values[k] = interpolated_val;
                        interpolated_bitmap[k] = true;
                    }
                }

                // Set the current non-null value
                let val: Option<f64> = values[j];
                interpolated_values[j] = val.unwrap();
                interpolated_bitmap[j] = true;
                last_non_null_idx = Some(j);
            }
            // If current is null, we'll handle it later
            // Trailing nulls remain as null
        }

        match self {
            Series::I32(_, _, _) => {
                let rounded: Vec<i32> = interpolated_values
                    .iter()
                    .map(|&v| v.round() as i32)
                    .collect();
                Ok(Series::I32(name, rounded, interpolated_bitmap))
            }
            Series::F64(_, _, _) => Ok(Series::F64(name, interpolated_values, interpolated_bitmap)),
            _ => unreachable!(),
        }
    }

    pub fn append(&self, other: &Series) -> Result<Self, VeloxxError> {
        if self.data_type() != other.data_type() {
            return Err(VeloxxError::DataTypeMismatch(format!(
                "Cannot append Series of different types: {:?} and {:?}",
                self.data_type(),
                other.data_type()
            )));
        }
        let new_name = self.name().to_string();
        match (self, other) {
            (Series::I32(_, values1, bitmap1), Series::I32(_, values2, bitmap2)) => {
                let mut new_values = values1.clone();
                let mut new_bitmap = bitmap1.clone();
                new_values.extend(values2.iter().cloned());
                new_bitmap.extend(bitmap2.iter().cloned());
                Ok(Series::I32(new_name, new_values, new_bitmap))
            }
            (Series::F64(_, values1, bitmap1), Series::F64(_, values2, bitmap2)) => {
                let mut new_values = values1.clone();
                let mut new_bitmap = bitmap1.clone();
                new_values.extend(values2.iter().cloned());
                new_bitmap.extend(bitmap2.iter().cloned());
                Ok(Series::F64(new_name, new_values, new_bitmap))
            }
            (Series::Bool(_, values1, bitmap1), Series::Bool(_, values2, bitmap2)) => {
                let mut new_values = values1.clone();
                let mut new_bitmap = bitmap1.clone();
                new_values.extend(values2.iter().cloned());
                new_bitmap.extend(bitmap2.iter().cloned());
                Ok(Series::Bool(new_name, new_values, new_bitmap))
            }
            (Series::String(_, values1, bitmap1), Series::String(_, values2, bitmap2)) => {
                let mut new_values = values1.clone();
                let mut new_bitmap = bitmap1.clone();
                new_values.extend(values2.iter().cloned());
                new_bitmap.extend(bitmap2.iter().cloned());
                Ok(Series::String(new_name, new_values, new_bitmap))
            }
            (Series::DateTime(_, values1, bitmap1), Series::DateTime(_, values2, bitmap2)) => {
                let mut new_values = values1.clone();
                let mut new_bitmap = bitmap1.clone();
                new_values.extend(values2.iter().cloned());
                new_bitmap.extend(bitmap2.iter().cloned());
                Ok(Series::DateTime(new_name, new_values, new_bitmap))
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Mismatched series types during append (should be caught by data_type check).".to_string()
            )),
        }
    }

    pub fn get_data_i32(&self) -> Result<Vec<Option<i32>>, VeloxxError> {
        match self {
            Series::I32(_, values, validity) => Ok(values
                .iter()
                .zip(validity.iter())
                .map(|(&v, &b)| if b { Some(v) } else { None })
                .collect()),
            _ => Err(VeloxxError::DataTypeMismatch(
                "Expected I32 series".to_string(),
            )),
        }
    }

    pub fn get_data_f64(&self) -> Result<Vec<Option<f64>>, VeloxxError> {
        match self {
            Series::F64(_, values, validity) => Ok(values
                .iter()
                .zip(validity.iter())
                .map(|(&v, &b)| if b { Some(v) } else { None })
                .collect()),
            _ => Err(VeloxxError::DataTypeMismatch(
                "Expected F64 series".to_string(),
            )),
        }
    }

    pub fn get_data_string(&self) -> Result<Vec<Option<String>>, VeloxxError> {
        match self {
            Series::String(_, values, validity) => Ok(values
                .iter()
                .zip(validity.iter())
                .map(|(v, &b)| if b { Some(v.clone()) } else { None })
                .collect()),
            _ => Err(VeloxxError::DataTypeMismatch(
                "Expected String series".to_string(),
            )),
        }
    }

    pub fn get_data_bool(&self) -> Result<Vec<Option<bool>>, VeloxxError> {
        match self {
            Series::Bool(_, values, validity) => Ok(values
                .iter()
                .zip(validity.iter())
                .map(|(&v, &b)| if b { Some(v) } else { None })
                .collect()),
            _ => Err(VeloxxError::DataTypeMismatch(
                "Expected Bool series".to_string(),
            )),
        }
    }

    pub fn get_data_datetime(&self) -> Result<Vec<Option<i64>>, VeloxxError> {
        match self {
            Series::DateTime(_, values, validity) => Ok(values
                .iter()
                .zip(validity.iter())
                .map(|(&v, &b)| if b { Some(v) } else { None })
                .collect()),
            _ => Err(VeloxxError::DataTypeMismatch(
                "Expected DateTime series".to_string(),
            )),
        }
    }

    /// Cast series to a different data type
    pub fn cast(&self, to_type: DataType) -> Result<Series, VeloxxError> {
        let name = self.name();
        let target_type = to_type.clone();
        match (self, to_type) {
            // I32 to F64
            (Series::I32(_, values, bitmap), DataType::F64) => {
                let new_values: Vec<f64> = values.iter().map(|&x| x as f64).collect();
                Ok(Series::F64(name.to_string(), new_values, bitmap.clone()))
            }
            // F64 to I32 (with truncation)
            (Series::F64(_, values, bitmap), DataType::I32) => {
                let new_values: Vec<i32> = values.iter().map(|&x| x as i32).collect();
                Ok(Series::I32(name.to_string(), new_values, bitmap.clone()))
            }
            // String to any numeric type (try parsing)
            (Series::String(_, values, bitmap), DataType::F64) => {
                let mut new_values = Vec::new();
                let mut new_bitmap = Vec::new();
                for (i, value) in values.iter().enumerate() {
                    if bitmap[i] {
                        match value.parse::<f64>() {
                            Ok(parsed) => {
                                new_values.push(parsed);
                                new_bitmap.push(true);
                            }
                            Err(_) => {
                                new_values.push(0.0);
                                new_bitmap.push(false);
                            }
                        }
                    } else {
                        new_values.push(0.0);
                        new_bitmap.push(false);
                    }
                }
                Ok(Series::F64(name.to_string(), new_values, new_bitmap))
            }
            // Same type - just clone
            (_, target_type) if self.data_type() == target_type => Ok(self.clone()),
            // Unsupported conversion
            _ => Err(VeloxxError::InvalidOperation(
                format!("Cannot cast from {:?} to {:?}", self.data_type(), target_type)
            )),
        }
    }

    /// Calculate correlation between two numeric series
    pub fn correlation(&self, other: &Series) -> Result<Option<f64>, VeloxxError> {
        // Both series must be numeric and same length
        if !self.is_numeric() || !other.is_numeric() {
            return Err(VeloxxError::InvalidOperation(
                "Correlation requires numeric series".to_string()
            ));
        }
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for correlation".to_string()
            ));
        }

        // Extract numeric values, skipping nulls
        let mut pairs = Vec::new();
        for i in 0..self.len() {
            if let (Some(x), Some(y)) = (self.get_numeric_f64(i), other.get_numeric_f64(i)) {
                pairs.push((x, y));
            }
        }

        if pairs.len() < 2 {
            return Ok(None); // Not enough data points
        }

        // Calculate means
        let mean_x: f64 = pairs.iter().map(|(x, _)| x).sum::<f64>() / pairs.len() as f64;
        let mean_y: f64 = pairs.iter().map(|(_, y)| y).sum::<f64>() / pairs.len() as f64;

        // Calculate correlation
        let mut numerator = 0.0;
        let mut sum_x_sq = 0.0;
        let mut sum_y_sq = 0.0;

        for (x, y) in pairs {
            let dx = x - mean_x;
            let dy = y - mean_y;
            numerator += dx * dy;
            sum_x_sq += dx * dx;
            sum_y_sq += dy * dy;
        }

        let denominator = (sum_x_sq * sum_y_sq).sqrt();
        if denominator == 0.0 {
            Ok(None) // No variance
        } else {
            Ok(Some(numerator / denominator))
        }
    }

    /// Calculate covariance between two numeric series
    pub fn covariance(&self, other: &Series) -> Result<Option<f64>, VeloxxError> {
        // Both series must be numeric and same length
        if !self.is_numeric() || !other.is_numeric() {
            return Err(VeloxxError::InvalidOperation(
                "Covariance requires numeric series".to_string()
            ));
        }
        if self.len() != other.len() {
            return Err(VeloxxError::InvalidOperation(
                "Series must have same length for covariance".to_string()
            ));
        }

        // Extract numeric values, skipping nulls
        let mut pairs = Vec::new();
        for i in 0..self.len() {
            if let (Some(x), Some(y)) = (self.get_numeric_f64(i), other.get_numeric_f64(i)) {
                pairs.push((x, y));
            }
        }

        if pairs.len() < 2 {
            return Ok(None); // Not enough data points
        }

        // Calculate means
        let mean_x: f64 = pairs.iter().map(|(x, _)| x).sum::<f64>() / pairs.len() as f64;
        let mean_y: f64 = pairs.iter().map(|(_, y)| y).sum::<f64>() / pairs.len() as f64;

        // Calculate covariance
        let covariance: f64 = pairs.iter()
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum::<f64>() / (pairs.len() - 1) as f64;

        Ok(Some(covariance))
    }

    /// Count unique values in the series
    pub fn unique_count(&self) -> Result<usize, VeloxxError> {
        let unique_series = self.unique()?;
        Ok(unique_series.len())
    }
}

pub mod aggregations;
pub mod arithmetic;
pub mod time_series;
pub mod ops;
