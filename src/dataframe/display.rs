use crate::{dataframe::DataFrame, series::Series};
use std::fmt;

/// Implements the `Display` trait for `DataFrame`.
///
/// This allows `DataFrame` instances to be pretty-printed to the console,
/// providing a human-readable tabular representation of the data.
///
/// The output includes column headers, a separator line, and then each row of data.
/// Null values are displayed as "null". Floating-point numbers are formatted to two decimal places.
/// Columns are sorted alphabetically by name for consistent display.
///
/// # Examples
///
/// ```rust
/// use veloxx::dataframe::DataFrame;
/// use veloxx::series::Series;
/// use std::collections::BTreeMap;
///
/// let mut columns = BTreeMap::new();
/// columns.insert("name".to_string(), Series::new_string("name", vec![Some("Alice".to_string()), Some("Bob".to_string())]));
/// columns.insert("age".to_string(), Series::new_i32("age", vec![Some(30), Some(24)]));
/// columns.insert("score".to_string(), Series::new_f64("score", vec![Some(85.5), Some(92.123)]));
///
/// let df = DataFrame::new(columns).unwrap();
/// println!("{}", df);
/// ```
///
/// This would print a formatted table similar to:
///
/// ```text
/// age            name           score          
/// --------------- --------------- ---------------
/// 30             Alice          85.50          
/// 24             Bob            92.12          
/// ```
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
                    Series::DateTime(_, v) => {
                        let val = v[i].map_or("null".to_string(), |x| x.to_string());
                        write!(f, "{val: <15}")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
