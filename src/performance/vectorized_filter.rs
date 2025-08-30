// src/performance/vectorized_filter.rs
use crate::performance::specialized_structures::BitPackedArray;
use crate::series::Series;
use crate::types::Value;
use crate::VeloxxError;

/// High-performance vectorized filtering operations
pub struct VectorizedFilter;

impl VectorizedFilter {
    /// Create a bit mask for filtering based on comparison with a scalar value
    pub fn create_comparison_mask_f64(
        values: &[f64],
        bitmap: &[bool],
        comparison_value: f64,
        op: ComparisonOp,
    ) -> Result<BitPackedArray, VeloxxError> {
        let mut mask = BitPackedArray::new(values.len());

        // Optimized comparison loop with better cache efficiency
        match op {
            ComparisonOp::Gt => {
                for i in 0..values.len() {
                    let result = bitmap[i] && values[i] > comparison_value;
                    mask.push(result);
                }
            }
            ComparisonOp::Gte => {
                for i in 0..values.len() {
                    let result = bitmap[i] && values[i] >= comparison_value;
                    mask.push(result);
                }
            }
            ComparisonOp::Lt => {
                for i in 0..values.len() {
                    let result = bitmap[i] && values[i] < comparison_value;
                    mask.push(result);
                }
            }
            ComparisonOp::Lte => {
                for i in 0..values.len() {
                    let result = bitmap[i] && values[i] <= comparison_value;
                    mask.push(result);
                }
            }
            ComparisonOp::Eq => {
                for i in 0..values.len() {
                    let result = bitmap[i] && (values[i] - comparison_value).abs() < f64::EPSILON;
                    mask.push(result);
                }
            }
            ComparisonOp::Ne => {
                for i in 0..values.len() {
                    let result = bitmap[i] && (values[i] - comparison_value).abs() >= f64::EPSILON;
                    mask.push(result);
                }
            }
        }

        Ok(mask)
    }

    /// Create a bit mask for filtering based on comparison with a scalar value (i32)
    pub fn create_comparison_mask_i32(
        values: &[i32],
        bitmap: &[bool],
        comparison_value: i32,
        op: ComparisonOp,
    ) -> Result<BitPackedArray, VeloxxError> {
        let mut mask = BitPackedArray::new(values.len());

        for i in 0..values.len() {
            let result = if bitmap[i] {
                match op {
                    ComparisonOp::Gt => values[i] > comparison_value,
                    ComparisonOp::Gte => values[i] >= comparison_value,
                    ComparisonOp::Lt => values[i] < comparison_value,
                    ComparisonOp::Lte => values[i] <= comparison_value,
                    ComparisonOp::Eq => values[i] == comparison_value,
                    ComparisonOp::Ne => values[i] != comparison_value,
                }
            } else {
                false
            };
            mask.push(result);
        }

        Ok(mask)
    }

    /// Fast filtering of a Series using a pre-computed bit mask
    pub fn filter_series_with_mask(
        series: &Series,
        mask: &BitPackedArray,
    ) -> Result<Series, VeloxxError> {
        match series {
            Series::F64(name, values, bitmap) => {
                if values.len() != mask.len() {
                    return Err(VeloxxError::InvalidOperation(
                        "Series and mask must have same length".to_string(),
                    ));
                }

                // Pre-allocate with estimated capacity for better performance
                let estimated_size = mask.count_ones().min(values.len() / 2);
                let mut filtered_values = Vec::with_capacity(estimated_size);
                let mut filtered_bitmap = Vec::with_capacity(estimated_size);

                for i in 0..values.len() {
                    if mask.get(i).unwrap_or(false) {
                        filtered_values.push(values[i]);
                        filtered_bitmap.push(bitmap[i]);
                    }
                }

                Ok(Series::F64(name.clone(), filtered_values, filtered_bitmap))
            }
            Series::I32(name, values, bitmap) => {
                if values.len() != mask.len() {
                    return Err(VeloxxError::InvalidOperation(
                        "Series and mask must have same length".to_string(),
                    ));
                }

                let estimated_size = mask.count_ones().min(values.len() / 2);
                let mut filtered_values = Vec::with_capacity(estimated_size);
                let mut filtered_bitmap = Vec::with_capacity(estimated_size);

                for i in 0..values.len() {
                    if mask.get(i).unwrap_or(false) {
                        filtered_values.push(values[i]);
                        filtered_bitmap.push(bitmap[i]);
                    }
                }

                Ok(Series::I32(name.clone(), filtered_values, filtered_bitmap))
            }
            Series::String(name, values, bitmap) => {
                if values.len() != mask.len() {
                    return Err(VeloxxError::InvalidOperation(
                        "Series and mask must have same length".to_string(),
                    ));
                }

                let estimated_size = mask.count_ones().min(values.len() / 2);
                let mut filtered_values = Vec::with_capacity(estimated_size);
                let mut filtered_bitmap = Vec::with_capacity(estimated_size);

                for i in 0..values.len() {
                    if mask.get(i).unwrap_or(false) {
                        filtered_values.push(values[i].clone());
                        filtered_bitmap.push(bitmap[i]);
                    }
                }

                Ok(Series::String(
                    name.clone(),
                    filtered_values,
                    filtered_bitmap,
                ))
            }
            Series::Bool(name, values, bitmap) => {
                if values.len() != mask.len() {
                    return Err(VeloxxError::InvalidOperation(
                        "Series and mask must have same length".to_string(),
                    ));
                }

                let estimated_size = mask.count_ones().min(values.len() / 2);
                let mut filtered_values = Vec::with_capacity(estimated_size);
                let mut filtered_bitmap = Vec::with_capacity(estimated_size);

                for i in 0..values.len() {
                    if mask.get(i).unwrap_or(false) {
                        filtered_values.push(values[i]);
                        filtered_bitmap.push(bitmap[i]);
                    }
                }

                Ok(Series::Bool(name.clone(), filtered_values, filtered_bitmap))
            }
            Series::DateTime(name, values, bitmap) => {
                if values.len() != mask.len() {
                    return Err(VeloxxError::InvalidOperation(
                        "Series and mask must have same length".to_string(),
                    ));
                }

                let estimated_size = mask.count_ones().min(values.len() / 2);
                let mut filtered_values = Vec::with_capacity(estimated_size);
                let mut filtered_bitmap = Vec::with_capacity(estimated_size);

                for i in 0..values.len() {
                    if mask.get(i).unwrap_or(false) {
                        filtered_values.push(values[i]);
                        filtered_bitmap.push(bitmap[i]);
                    }
                }

                Ok(Series::DateTime(
                    name.clone(),
                    filtered_values,
                    filtered_bitmap,
                ))
            }
        }
    }

    /// High-performance single-column filter operation
    pub fn fast_filter_single_column(
        series: &Series,
        comparison_value: &Value,
        op: ComparisonOp,
    ) -> Result<BitPackedArray, VeloxxError> {
        match (series, comparison_value) {
            (Series::F64(_, values, bitmap), Value::F64(cmp_val)) => {
                Self::create_comparison_mask_f64(values, bitmap, *cmp_val, op)
            }
            (Series::I32(_, values, bitmap), Value::I32(cmp_val)) => {
                Self::create_comparison_mask_i32(values, bitmap, *cmp_val, op)
            }
            (Series::String(_, values, bitmap), Value::String(cmp_val)) => {
                Self::create_comparison_mask_string(values, bitmap, cmp_val, op)
            }
            _ => Err(VeloxxError::Unsupported(
                "Unsupported combination for fast filtering".to_string(),
            )),
        }
    }

    /// Create a bit mask for filtering based on string comparison
    pub fn create_comparison_mask_string(
        values: &[String],
        bitmap: &[bool],
        comparison_value: &str,
        op: ComparisonOp,
    ) -> Result<BitPackedArray, VeloxxError> {
        let mut mask = BitPackedArray::new(values.len());

        for i in 0..values.len() {
            let result = if bitmap[i] {
                match op {
                    ComparisonOp::Eq => values[i].as_str() == comparison_value,
                    ComparisonOp::Ne => values[i].as_str() != comparison_value,
                    ComparisonOp::Gt => values[i].as_str() > comparison_value,
                    ComparisonOp::Gte => values[i].as_str() >= comparison_value,
                    ComparisonOp::Lt => values[i].as_str() < comparison_value,
                    ComparisonOp::Lte => values[i].as_str() <= comparison_value,
                }
            } else {
                false // null values don't match
            };
            mask.push(result);
        }

        Ok(mask)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComparisonOp {
    Gt,  // >
    Gte, // >=
    Lt,  // <
    Lte, // <=
    Eq,  // ==
    Ne,  // !=
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_mask_f64() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let bitmap = [true, true, true, true, true];

        let mask =
            VectorizedFilter::create_comparison_mask_f64(&values, &bitmap, 3.0, ComparisonOp::Gt)
                .unwrap();

        assert_eq!(mask.get(0), Some(false)); // 1.0 > 3.0 = false
        assert_eq!(mask.get(1), Some(false)); // 2.0 > 3.0 = false
        assert_eq!(mask.get(2), Some(false)); // 3.0 > 3.0 = false
        assert_eq!(mask.get(3), Some(true)); // 4.0 > 3.0 = true
        assert_eq!(mask.get(4), Some(true)); // 5.0 > 3.0 = true
        assert_eq!(mask.count_ones(), 2);
    }

    #[test]
    fn test_filter_series_with_mask() {
        let series = Series::F64(
            "test".to_string(),
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![true, true, true, true, true],
        );

        let mask = VectorizedFilter::fast_filter_single_column(
            &series,
            &Value::F64(3.0),
            ComparisonOp::Gt,
        )
        .unwrap();

        assert_eq!(mask.count_ones(), 2); // 4.0 and 5.0 are > 3.0
    }

    #[test]
    fn test_string_comparison_mask() {
        let values = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        let bitmap = [true, true, true];

        let mask = VectorizedFilter::create_comparison_mask_string(
            &values,
            &bitmap,
            "banana",
            ComparisonOp::Eq,
        )
        .unwrap();

        assert_eq!(mask.get(0), Some(false)); // "apple" == "banana" = false
        assert_eq!(mask.get(1), Some(true)); // "banana" == "banana" = true
        assert_eq!(mask.get(2), Some(false)); // "cherry" == "banana" = false
        assert_eq!(mask.count_ones(), 1);
    }
}
