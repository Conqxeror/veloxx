
use crate::series::Series;
use crate::error::VeloxxError;

impl Series {
    pub fn str_contains(&self, pat: &str) -> Result<Series, VeloxxError> {
        if let Series::String(name, data) = self {
            let new_data = data.iter().map(|opt| opt.as_ref().map(|s| s.contains(pat))).collect();
            Ok(Series::Bool(name.clone(), new_data))
        } else {
            Err(VeloxxError::invalid_operation("str_contains can only be used on a String series"))
        }
    }
}
