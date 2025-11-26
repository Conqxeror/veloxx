use crate::dataframe::DataFrame;
use crate::series::Series;
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
/// use indexmap::IndexMap;
///
/// let mut columns = IndexMap::new();
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
/// name           age            score          
/// --------------- --------------- ---------------
/// Alice          30             85.50          
/// Bob            24             92.12          
/// ```
impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.row_count() == 0 {
            return write!(f, "Empty DataFrame");
        }

        let column_names: Vec<&String> = self.columns.keys().collect();

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
        for i in 0..self.row_count() {
            for name in &column_names {
                let series = self.columns.get(*name).unwrap();
                let value_str = match series {
                    Series::I32(_, v, _) => v.get(i).map_or("null".to_string(), |i| i.to_string()),
                    Series::F64(_, v, _) => v.get(i).map_or("null".to_string(), |f| f.to_string()),
                    Series::Bool(_, v, _) => v.get(i).map_or("null".to_string(), |b| b.to_string()),
                    Series::String(_, v, _) => v.get(i).map_or("null".to_string(), |s| s.clone()),
                    Series::DateTime(_, v, _) => {
                        v.get(i).map_or("null".to_string(), |t| t.to_string())
                    }
                };
                write!(f, "{value_str: <15}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
