use crate::series::Series;
use crate::types::Value;
use crate::VeloxxError;


impl Series {
    /// Filter the series to only include values at the specified indices
    pub fn filter(&self, indices: &[usize]) -> Result<Series, VeloxxError> {
        match self {
            Series::I32(name, values, bitmap) => {
                let mut new_values = Vec::with_capacity(indices.len());
                let mut new_bitmap = Vec::with_capacity(indices.len());
                
                for &idx in indices {
                    if idx < values.len() {
                        new_values.push(values[idx]);
                        new_bitmap.push(bitmap[idx]);
                    } else {
                        return Err(VeloxxError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                }
                
                Ok(Series::I32(name.clone(), new_values, new_bitmap))
            }
            Series::F64(name, values, bitmap) => {
                let mut new_values = Vec::with_capacity(indices.len());
                let mut new_bitmap = Vec::with_capacity(indices.len());
                
                for &idx in indices {
                    if idx < values.len() {
                        new_values.push(values[idx]);
                        new_bitmap.push(bitmap[idx]);
                    } else {
                        return Err(VeloxxError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                }
                
                Ok(Series::F64(name.clone(), new_values, new_bitmap))
            }
            Series::Bool(name, values, bitmap) => {
                let mut new_values = Vec::with_capacity(indices.len());
                let mut new_bitmap = Vec::with_capacity(indices.len());
                
                for &idx in indices {
                    if idx < values.len() {
                        new_values.push(values[idx]);
                        new_bitmap.push(bitmap[idx]);
                    } else {
                        return Err(VeloxxError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                }
                
                Ok(Series::Bool(name.clone(), new_values, new_bitmap))
            }
            Series::String(name, values, bitmap) => {
                let mut new_values = Vec::with_capacity(indices.len());
                let mut new_bitmap = Vec::with_capacity(indices.len());
                
                for &idx in indices {
                    if idx < values.len() {
                        new_values.push(values[idx].clone());
                        new_bitmap.push(bitmap[idx]);
                    } else {
                        return Err(VeloxxError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                }
                
                Ok(Series::String(name.clone(), new_values, new_bitmap))
            }
            Series::DateTime(name, values, bitmap) => {
                let mut new_values = Vec::with_capacity(indices.len());
                let mut new_bitmap = Vec::with_capacity(indices.len());
                
                for &idx in indices {
                    if idx < values.len() {
                        new_values.push(values[idx]);
                        new_bitmap.push(bitmap[idx]);
                    } else {
                        return Err(VeloxxError::InvalidOperation(
                            "Index out of bounds".to_string(),
                        ));
                    }
                }
                
                Ok(Series::DateTime(name.clone(), new_values, new_bitmap))
            }
        }
    }

    /// Convert series to vector of f64 values (for numeric series)
    pub fn to_vec_f64(&self) -> Result<Vec<f64>, VeloxxError> {
        match self {
            Series::I32(_, values, bitmap) => {
                Ok(values
                    .iter()
                    .zip(bitmap.iter())
                    .filter_map(|(&v, &b)| if b { Some(v as f64) } else { None })
                    .collect())
            }
            Series::F64(_, values, bitmap) => {
                Ok(values
                    .iter()
                    .zip(bitmap.iter())
                    .filter_map(|(&v, &b)| if b { Some(v) } else { None })
                    .collect())
            }
            _ => Err(VeloxxError::InvalidOperation(
                "Cannot convert to f64 vector for this data type".to_string(),
            )),
        }
    }

    /// Set the name of the series
    pub fn set_name(&mut self, new_name: &str) {
        match self {
            Series::I32(ref mut name, _, _) => *name = new_name.to_string(),
            Series::F64(ref mut name, _, _) => *name = new_name.to_string(),
            Series::Bool(ref mut name, _, _) => *name = new_name.to_string(),
            Series::String(ref mut name, _, _) => *name = new_name.to_string(),
            Series::DateTime(ref mut name, _, _) => *name = new_name.to_string(),
        }
    }

    /// Count the number of valid (non-null) values in the series
    pub fn count(&self) -> usize {
        match self {
            Series::I32(_, _, bitmap) => bitmap.iter().filter(|&&b| b).count(),
            Series::F64(_, _, bitmap) => bitmap.iter().filter(|&&b| b).count(),
            Series::Bool(_, _, bitmap) => bitmap.iter().filter(|&&b| b).count(),
            Series::String(_, _, bitmap) => bitmap.iter().filter(|&&b| b).count(),
            Series::DateTime(_, _, bitmap) => bitmap.iter().filter(|&&b| b).count(),
        }
    }

    /// Fill null values with a specified value
    pub fn fill_nulls(&self, value: &Value) -> Result<Series, VeloxxError> {
        let name = self.name().to_string();
        
        match (self, value) {
            (Series::I32(_, values, bitmap), Value::I32(fill_value)) => {
                let mut new_values = values.clone();
                let new_bitmap = vec![true; values.len()];
                
                for (i, &is_valid) in bitmap.iter().enumerate() {
                    if !is_valid {
                        new_values[i] = *fill_value;
                    }
                }
                
                Ok(Series::I32(name, new_values, new_bitmap))
            }
            (Series::F64(_, values, bitmap), Value::F64(fill_value)) => {
                let mut new_values = values.clone();
                let new_bitmap = vec![true; values.len()];
                
                for (i, &is_valid) in bitmap.iter().enumerate() {
                    if !is_valid {
                        new_values[i] = *fill_value;
                    }
                }
                
                Ok(Series::F64(name, new_values, new_bitmap))
            }
            (Series::Bool(_, values, bitmap), Value::Bool(fill_value)) => {
                let mut new_values = values.clone();
                let new_bitmap = vec![true; values.len()];
                
                for (i, &is_valid) in bitmap.iter().enumerate() {
                    if !is_valid {
                        new_values[i] = *fill_value;
                    }
                }
                
                Ok(Series::Bool(name, new_values, new_bitmap))
            }
            (Series::String(_, values, bitmap), Value::String(fill_value)) => {
                let mut new_values = values.clone();
                let new_bitmap = vec![true; values.len()];
                
                for (i, &is_valid) in bitmap.iter().enumerate() {
                    if !is_valid {
                        new_values[i] = fill_value.clone();
                    }
                }
                
                Ok(Series::String(name, new_values, new_bitmap))
            }
            _ => Err(VeloxxError::DataTypeMismatch(
                "Cannot fill nulls: data type mismatch".to_string(),
            )),
        }
    }
}