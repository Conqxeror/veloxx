pub struct GlobalAggregate;

impl GlobalAggregate {
    pub fn sum_f64(data: &[f64]) -> f64 {
        data.iter().copied().sum()
    }
    pub fn mean_f64(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().copied().sum::<f64>() / data.len() as f64)
        }
    }
}
