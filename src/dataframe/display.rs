use crate::{dataframe::DataFrame, series::Series};
use std::fmt;

/// Implements the `Display` trait for `DataFrame`.
///
/// This allows `DataFrame` instances to be pretty-printed to the console,
/// providing a human-readable tabular representation of the data.
impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.row_count == 0 {
            return write!(f, "Empty DataFrame");
        }

        let mut column_names: Vec<&String> = self.columns.keys().collect();
        column_names.sort_unstable(); // Ensure consistent column order

        // Print header
        for name in &column_names {
            write!(f, "{name: <15}")?;
        }
        writeln!(f)?;
        for _ in &column_names {
            write!(f, "--------------- ")?;
        }
        writeln!(f)?;

        // Print data
        for i in 0..self.row_count {
            for name in &column_names {
                let series = self.columns.get(*name).unwrap();
                match series {
                    Series::I32(_, v) => {
                        let val = v[i].map_or("null".to_string(), |x| x.to_string());
                        write!(f, "{val: <15}")?;
                    }
                    Series::F64(_, v) => {
                        let val = v[i].map_or("null".to_string(), |x| format!("{x:.2}"));
                        write!(f, "{val: <15}")?;
                    }
                    Series::Bool(_, v) => {
                        let val = v[i].map_or("null".to_string(), |x| x.to_string());
                        write!(f, "{val: <15}")?;
                    }
                    Series::String(_, v) => {
                        let val = v[i].as_ref().map_or("null".to_string(), |x| x.clone());
                        write!(f, "{val: <15}")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
