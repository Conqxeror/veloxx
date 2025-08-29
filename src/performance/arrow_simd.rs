//! Direct SIMD operations on Arrow arrays

#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
use arrow_array::{Float64Array, Int32Array};

#[cfg(all(feature = "simd", target_arch = "aarch64"))]
use std::arch::aarch64::*;
#[cfg(all(feature = "simd", target_arch = "x86_64", not(target_arch = "wasm32")))]
use wide::{f64x4, i32x4};

/// Perform SIMD addition directly on Arrow Float64Array
#[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
#[cfg(feature = "simd")]
pub fn simd_add_f64_arrays(arr1: &Float64Array, arr2: &Float64Array) -> Float64Array {
    #[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
    use arrow_arith::numeric;
    #[cfg(all(feature = "arrow", not(target_arch = "wasm32")))]
    use arrow_array::Float64Array;
    let result = numeric::add(arr1, arr2).unwrap();
    let array = result.as_any().downcast_ref::<Float64Array>().unwrap();
    array.clone()
}

/// Perform SIMD addition directly on Arrow Int32Array
#[cfg(feature = "arrow")]
#[cfg(feature = "simd")]
pub fn simd_add_i32_arrays(arr1: &Int32Array, arr2: &Int32Array) -> Int32Array {
    assert_eq!(arr1.len(), arr2.len(), "Arrays must have same length");

    let len = arr1.len();
    let mut result = Vec::with_capacity(len);
    let simd_len = len / 4;
    let remainder = len % 4;

    // Get direct access to the underlying data
    let values1 = arr1.values();
    let values2 = arr2.values();

    // Process 4 elements at a time using SIMD
    for i in 0..simd_len {
        let a = i32x4::from([
            values1[i * 4],
            values1[i * 4 + 1],
            values1[i * 4 + 2],
            values1[i * 4 + 3],
        ]);
        let b = i32x4::from([
            values2[i * 4],
            values2[i * 4 + 1],
            values2[i * 4 + 2],
            values2[i * 4 + 3],
        ]);
        let res = a + b;
        let res_array = res.to_array();
        result.extend_from_slice(&res_array);
    }

    // Process remaining elements
    for i in (len - remainder)..len {
        result.push(values1[i] + values2[i]);
    }

    Int32Array::from(result)
}

/// Perform SIMD sum directly on Arrow Float64Array
#[cfg(feature = "arrow")]
#[cfg(feature = "simd")]
pub fn simd_sum_f64_array(arr: &Float64Array) -> f64 {
    let len = arr.len();
    let simd_len = len / 4;
    let remainder = len % 4;
    let mut simd_sum = f64x4::ZERO;

    // Get direct access to the underlying data
    let values = arr.values();

    // Process 4 elements at a time using SIMD
    for i in 0..simd_len {
        let a = f64x4::from([
            values[i * 4],
            values[i * 4 + 1],
            values[i * 4 + 2],
            values[i * 4 + 3],
        ]);
        simd_sum += a;
    }

    // Sum the SIMD result
    let mut sum = simd_sum.reduce_add();

    // Add remaining elements
    for i in (len - remainder)..len {
        sum += values[i];
    }

    sum
}

/// Perform SIMD sum directly on Arrow Int32Array
#[cfg(feature = "arrow")]
#[cfg(feature = "simd")]
pub fn simd_sum_i32_array(arr: &Int32Array) -> i32 {
    let len = arr.len();
    let simd_len = len / 4;
    let remainder = len % 4;
    let mut simd_sum = i32x4::ZERO;

    // Get direct access to the underlying data
    let values = arr.values();

    // Process 4 elements at a time using SIMD
    for i in 0..simd_len {
        let a = i32x4::from([
            values[i * 4],
            values[i * 4 + 1],
            values[i * 4 + 2],
            values[i * 4 + 3],
        ]);
        simd_sum += a;
    }

    // Sum the SIMD result
    let mut sum = simd_sum.reduce_add();

    // Add remaining elements
    for i in (len - remainder)..len {
        sum += values[i];
    }

    sum
}

#[cfg(all(test, feature = "arrow", feature = "simd"))]
mod tests {
    use super::*;
    use arrow_array::Float64Array;
    use arrow_array::Int32Array;

    #[test]
    fn test_simd_add_f64_arrays() {
        let arr1 = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let arr2 = Float64Array::from(vec![1.0, 1.0, 1.0, 1.0, 1.0]);
        let result = simd_add_f64_arrays(&arr1, &arr2);
        assert_eq!(result.values(), &[2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_simd_add_i32_arrays() {
        let arr1 = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let arr2 = Int32Array::from(vec![1, 1, 1, 1, 1]);
        let result = simd_add_i32_arrays(&arr1, &arr2);
        assert_eq!(result.values(), &[2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_simd_sum_f64_array() {
        let arr = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = simd_sum_f64_array(&arr);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_simd_sum_i32_array() {
        let arr = Int32Array::from(vec![1, 2, 3, 4, 5]);
        let result = simd_sum_i32_array(&arr);
        assert_eq!(result, 15);
    }
}
