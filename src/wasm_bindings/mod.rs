

#![allow(clippy::boxed_local)]

use crate::dataframe::join::JoinType;
use std::collections::BTreeMap;
use crate::dataframe::DataFrame;
use crate::expressions::Expr;

use crate::types::{DataType, Value};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmDataFrame {
    df: DataFrame,
}

#[wasm_bindgen]
impl WasmDataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(columns: &js_sys::Object) -> Result<WasmDataFrame, JsValue> {
        let rust_columns = BTreeMap::new();
        let entries = js_sys::Object::entries(columns);
        for entry in entries.iter() {
            let arr = js_sys::Array::from(&entry);
            let _name = arr
                .get(0)
                .as_string()
                .ok_or("Column name must be a string")?;
            let _wasm_series: &js_sys::Object = &arr.into();
            // rust_columns.insert(name, wasm_series.series);
        }

        let df = DataFrame::new(rust_columns).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmDataFrame { df })
    }

    #[wasm_bindgen(js_name = rowCount)]
    pub fn row_count(&self) -> usize {
        self.df.row_count()
    }

    #[wasm_bindgen(js_name = columnCount)]
    pub fn column_count(&self) -> usize {
        self.df.column_count()
    }

    #[wasm_bindgen(js_name = columnNames)]
    pub fn column_names(&self) -> Box<[JsValue]> {
        self.df
            .column_names()
            .into_iter()
            .map(|s| JsValue::from_str(s))
            .collect::<Vec<JsValue>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(js_name = getColumn)]
    pub fn get_column(&self, name: &str) -> Option<js_sys::Object> {
        self.df.get_column(name).map(|_s| {
            js_sys::Object::new()
        })
    }

    #[wasm_bindgen]
    pub fn filter(&self, row_indices: Box<[usize]>) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .filter_by_indices(row_indices.as_ref())
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = selectColumns)]
    pub fn select_columns(&self, names: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let string_names: Vec<String> = names
            .iter()
            .map(|v| v.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .select_columns(string_names)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = dropColumns)]
    pub fn drop_columns(&self, names: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let string_names: Vec<String> = names
            .iter()
            .map(|v| v.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .drop_columns(string_names)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = renameColumn)]
    pub fn rename_column(&self, old_name: &str, new_name: &str) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .rename_column(old_name, new_name)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = dropNulls)]
    pub fn drop_nulls(&self, subset: Option<Box<[JsValue]>>) -> Result<WasmDataFrame, JsValue> {
        let subset_cols: Option<Vec<String>> = subset.map(|s| {
            s.iter()
                .map(|v| v.as_string().unwrap_or_default())
                .collect()
        });
        Ok(WasmDataFrame {
            df: self
                .df
                .drop_nulls(subset_cols.as_deref())
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = fillNulls)]
    pub fn fill_nulls(&self, value: &WasmValue) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .fill_nulls(value.value.clone())
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = fromCsv)]
    pub fn from_csv(path: &str) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: DataFrame::from_csv(path).map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(path: &str) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: DataFrame::from_json(path).map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = toCsv)]
    pub fn to_csv(&self, path: &str) -> Result<(), JsValue> {
        self.df
            .to_csv(path)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn join(
        &self,
        other: &WasmDataFrame,
        on_column: &str,
        join_type: WasmJoinType,
    ) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .join(&other.df, on_column, join_type.into())
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = groupBy)]
    pub fn group_by(&self, by_columns: Box<[JsValue]>) -> Result<WasmGroupedDataFrame, JsValue> {
        let string_cols: Vec<String> = by_columns
            .iter()
            .map(|v| v.as_string().unwrap_or_default())
            .collect();
        Ok(WasmGroupedDataFrame {
            dataframe: self.df.clone(),
            group_columns: string_cols,
        })
    }

    #[wasm_bindgen(js_name = withColumn)]
    pub fn with_column(
        &self,
        new_col_name: &str,
        expr: &WasmExpr,
    ) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .with_column(new_col_name, &expr.expr)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen]
    pub fn describe(&self) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .describe()
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen]
    pub fn correlation(&self, col1_name: &str, col2_name: &str) -> Result<f64, JsValue> {
        self.df
            .correlation(col1_name, col2_name)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn covariance(&self, col1_name: &str, col2_name: &str) -> Result<f64, JsValue> {
        self.df
            .covariance(col1_name, col2_name)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn append(&self, other: &WasmDataFrame) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .append(&other.df)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen]
    pub fn sort(
        &self,
        by_columns: Box<[JsValue]>,
        ascending: bool,
    ) -> Result<WasmDataFrame, JsValue> {
        let string_cols: Vec<String> = by_columns
            .iter()
            .map(|v| v.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .sort(string_cols, ascending)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    
    
}

#[wasm_bindgen]
pub enum WasmJoinType {
    Inner,
    Left,
    Right,
}

impl From<WasmJoinType> for JoinType {
    fn from(join_type: WasmJoinType) -> Self {
        match join_type {
            WasmJoinType::Inner => JoinType::Inner,
            WasmJoinType::Left => JoinType::Left,
            WasmJoinType::Right => JoinType::Right,
        }
    }
}







#[wasm_bindgen]
pub enum WasmDataType {
    I32,
    F64,
    Bool,
    String,
    DateTime,
}

impl From<DataType> for WasmDataType {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::I32 => WasmDataType::I32,
            DataType::F64 => WasmDataType::F64,
            DataType::Bool => WasmDataType::Bool,
            DataType::String => WasmDataType::String,
            DataType::DateTime => WasmDataType::DateTime,
        }
    }
}

impl From<WasmDataType> for DataType {
    fn from(wasm_data_type: WasmDataType) -> Self {
        match wasm_data_type {
            WasmDataType::I32 => DataType::I32,
            WasmDataType::F64 => DataType::F64,
            WasmDataType::Bool => DataType::Bool,
            WasmDataType::String => DataType::String,
            WasmDataType::DateTime => DataType::DateTime,
        }
    }
}

#[wasm_bindgen]
pub struct WasmValue {
    value: Value,
}

#[wasm_bindgen]
impl WasmValue {
    #[wasm_bindgen(constructor)]
    pub fn new(value: JsValue) -> Result<WasmValue, JsValue> {
        if value.is_falsy() && !value.is_null() && !value.is_undefined() {
            // Treat 0, empty string, false as their actual values, not null
            if let Some(v) = value.as_f64() {
                Ok(WasmValue {
                    value: Value::F64(v),
                })
            } else if let Some(v) = value.as_bool() {
                Ok(WasmValue {
                    value: Value::Bool(v),
                })
            } else if let Some(v) = value.as_string() {
                Ok(WasmValue {
                    value: Value::String(v),
                })
            } else {
                Err(JsValue::from_str("Unsupported WasmValue type"))
            }
        } else if value.is_null() || value.is_undefined() {
            Ok(WasmValue { value: Value::Null })
        } else if let Some(v) = value.as_f64() {
            // Check for integer specifically
            if v.fract() == 0.0 && v >= (i32::MIN as f64) && v <= (i32::MAX as f64) {
                Ok(WasmValue {
                    value: Value::I32(v as i32),
                })
            } else {
                Ok(WasmValue {
                    value: Value::F64(v),
                })
            }
        } else if let Some(v) = value.as_bool() {
            Ok(WasmValue {
                value: Value::Bool(v),
            })
        } else if let Some(v) = value.as_string() {
            Ok(WasmValue {
                value: Value::String(v),
            })
        } else {
            Err(JsValue::from_str("Unsupported WasmValue type"))
        }
    }

    pub fn to_js_value(&self) -> JsValue {
        match &self.value {
            Value::I32(v) => JsValue::from_f64(*v as f64),
            Value::F64(v) => JsValue::from_f64(*v),
            Value::Bool(v) => JsValue::from_bool(*v),
            Value::String(v) => JsValue::from_str(v),
            Value::DateTime(v) => JsValue::from_f64(*v as f64),
            Value::Null => JsValue::NULL,
        }
    }
}

#[wasm_bindgen(js_name = WasmGroupedDataFrame)]
pub struct WasmGroupedDataFrame {
    dataframe: DataFrame,
    group_columns: Vec<String>,
}

#[wasm_bindgen]
impl WasmGroupedDataFrame {
    #[wasm_bindgen]
    pub fn agg(&self, aggregations: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let rust_aggregations: Vec<(String, String)> = IntoIterator::into_iter(aggregations)
            .map(|js_val| {
                let arr = js_sys::Array::from(&js_val);
                let col = arr.get(0).as_string().unwrap_or_default();
                let agg = arr.get(1).as_string().unwrap_or_default();
                (col, agg)
            })
            .collect();
        // Convert to the expected format
        let string_refs: Vec<(&str, &str)> = rust_aggregations
            .iter()
            .map(|(col, agg)| (col.as_str(), agg.as_str()))
            .collect();

        let grouped_df = self
            .dataframe
            .group_by(self.group_columns.clone())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(WasmDataFrame {
            df: grouped_df
                .agg(string_refs)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }
}

#[wasm_bindgen(js_name = WasmExpr)]
pub struct WasmExpr {
    expr: Expr,
}

#[wasm_bindgen]
impl WasmExpr {
    #[wasm_bindgen(js_name = column)]
    pub fn column(name: &str) -> WasmExpr {
        WasmExpr {
            expr: Expr::Column(name.to_string()),
        }
    }

    #[wasm_bindgen(js_name = literal)]
    pub fn literal(value: &WasmValue) -> WasmExpr {
        WasmExpr {
            expr: Expr::Literal(value.value.clone()),
        }
    }

    #[wasm_bindgen(js_name = add)]
    pub fn add(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Add(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = subtract)]
    pub fn subtract(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Subtract(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = multiply)]
    pub fn multiply(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Multiply(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = divide)]
    pub fn divide(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Divide(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = equals)]
    pub fn equals(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Equals(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = notEquals)]
    pub fn not_equals(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::NotEquals(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = greaterThan)]
    pub fn greater_than(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::GreaterThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = lessThan)]
    pub fn less_than(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::LessThan(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = greaterThanOrEqual)]
    pub fn greater_than_or_equal(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::GreaterThanOrEqual(
                Box::new(left.expr.clone()),
                Box::new(right.expr.clone()),
            ),
        }
    }

    #[wasm_bindgen(js_name = lessThanOrEqual)]
    pub fn less_than_or_equal(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::LessThanOrEqual(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = and)]
    pub fn and(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::And(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = or)]
    pub fn or(left: &WasmExpr, right: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Or(Box::new(left.expr.clone()), Box::new(right.expr.clone())),
        }
    }

    #[wasm_bindgen(js_name = not)]
    pub fn not(expr: &WasmExpr) -> WasmExpr {
        WasmExpr {
            expr: Expr::Not(Box::new(expr.expr.clone())),
        }
    }
}

use std::fmt;

impl fmt::Display for WasmDataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.df)
    }
}