// src/performance/expression_fusion.rs
use crate::VeloxxError;

/// Fused expression operations for better performance
pub struct ExpressionFusion;

impl ExpressionFusion {
    /// Fused add and multiply: (a + b) * c
    pub fn fused_add_mul_f64(
        a: &[f64],
        b: &[f64],
        c: &[f64],
        result: &mut [f64],
    ) -> Result<(), VeloxxError> {
        if a.len() != b.len() || b.len() != c.len() || c.len() != result.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have the same length".to_string(),
            ));
        }

        use rayon::prelude::*;
        result.par_iter_mut().enumerate().for_each(|(i, r)| {
            *r = (a[i] + b[i]) * c[i];
        });

        Ok(())
    }

    /// Fused filter and sum: sum elements where condition is true
    pub fn fused_filter_sum_f64(values: &[f64], condition: &[bool]) -> Result<f64, VeloxxError> {
        if values.len() != condition.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have the same length".to_string(),
            ));
        }

        use rayon::prelude::*;
        let sum = values
            .par_iter()
            .zip(condition.par_iter())
            .filter_map(|(&v, &cond)| if cond { Some(v) } else { None })
            .sum::<f64>();
        Ok(sum)
    }

    /// Fused comparison and count: count elements where value > threshold
    pub fn fused_gt_count_f64(values: &[f64], threshold: f64) -> usize {
        values.iter().filter(|&&x| x > threshold).count()
    }

    /// Fused multiply and accumulate: result += a\[i\] * b\[i\]
    pub fn fused_multiply_accumulate_f64(a: &[f64], b: &[f64]) -> Result<f64, VeloxxError> {
        if a.len() != b.len() {
            return Err(VeloxxError::InvalidOperation(
                "Arrays must have the same length".to_string(),
            ));
        }

        let mut result = 0.0;
        for i in 0..a.len() {
            result += a[i] * b[i];
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fused_add_mul() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [1.0, 1.0, 1.0, 1.0];
        let c = [2.0, 2.0, 2.0, 2.0];
        let mut result = [0.0; 4];

        ExpressionFusion::fused_add_mul_f64(&a, &b, &c, &mut result).unwrap();
        assert_eq!(result, [4.0, 6.0, 8.0, 10.0]); // (1+1)*2, (2+1)*2, etc.
    }

    #[test]
    fn test_fused_filter_sum() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let condition = [true, false, true, false, true];

        let result = ExpressionFusion::fused_filter_sum_f64(&values, &condition).unwrap();
        assert_eq!(result, 9.0); // 1.0 + 3.0 + 5.0
    }

    #[test]
    fn test_fused_gt_count() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ExpressionFusion::fused_gt_count_f64(&values, 3.0);
        assert_eq!(result, 2); // 4.0 and 5.0 are > 3.0
    }

    #[test]
    fn test_fused_multiply_accumulate() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [2.0, 3.0, 4.0, 5.0];

        let result = ExpressionFusion::fused_multiply_accumulate_f64(&a, &b).unwrap();
        assert_eq!(result, 40.0); // 1*2 + 2*3 + 3*4 + 4*5 = 2 + 6 + 12 + 20 = 40
    }
}
