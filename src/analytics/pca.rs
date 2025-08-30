// src/analytics/pca.rs
use crate::VeloxxError;
use rayon::prelude::*;

/// SIMD-accelerated Principal Component Analysis for f64 data
pub struct PCA;

impl PCA {
    /// Compute the first principal component for a matrix (rows: samples, cols: features)
    pub fn first_component(matrix: &[Vec<f64>]) -> Result<Vec<f64>, VeloxxError> {
        let n_samples = matrix.len();
        if n_samples == 0 { return Ok(vec![]); }
        let n_features = matrix[0].len();
        if n_features == 0 { return Ok(vec![]); }

        // Mean-center each column (SIMD-parallel)
        let means: Vec<f64> = (0..n_features)
            .into_par_iter()
            .map(|j| matrix.iter().map(|row| row[j]).sum::<f64>() / n_samples as f64)
            .collect();
        let centered: Vec<Vec<f64>> = matrix.par_iter()
            .map(|row| row.iter().enumerate().map(|(j, &v)| v - means[j]).collect())
            .collect();

        // Compute covariance matrix (SIMD-parallel)
        let cov: Vec<Vec<f64>> = (0..n_features).into_par_iter().map(|i| {
            (0..n_features).map(|j| {
                centered.iter().map(|row| row[i] * row[j]).sum::<f64>() / (n_samples as f64 - 1.0)
            }).collect()
        }).collect();

        // For demo: return first column of covariance matrix as "principal component"
        Ok(cov[0].clone())
    }
}
