use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::types::{DataType, Value};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = WasmSeries)]
pub struct WasmSeries {
    series: Series,
}

#[wasm_bindgen]
impl WasmSeries {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str, data: Box<[JsValue]>) -> Result<WasmSeries, JsValue> {
        let mut i32_data = Vec::new();
        let mut f64_data = Vec::new();
        let mut bool_data = Vec::new();
        let mut string_data = Vec::new();
        let mut datetime_data = Vec::new();
        let mut inferred_type: Option<DataType> = None;
        for item in data.iter() {
            if item.is_null() || item.is_undefined() {
                i32_data.push(None);
                f64_data.push(None);
                bool_data.push(None);
                string_data.push(None);
                datetime_data.push(None);
            } else if item.as_f64().is_some() {
                if inferred_type.is_none() {
                    inferred_type = Some(DataType::F64);
                }
                f64_data.push(item.as_f64());
                i32_data.push(item.as_f64().map(|v| v as i32));
                bool_data.push(None);
                string_data.push(None);
                datetime_data.push(item.as_f64().map(|v| v as i64));
            } else if item.as_bool().is_some() {
                if inferred_type.is_none() {
                    inferred_type = Some(DataType::Bool);
                }
                bool_data.push(item.as_bool());
                i32_data.push(None);
                f64_data.push(None);
                string_data.push(None);
                datetime_data.push(None);
            } else if item.as_string().is_some() {
                if inferred_type.is_none() {
                    inferred_type = Some(DataType::String);
                }
                string_data.push(item.as_string());
                i32_data.push(None);
                f64_data.push(None);
                bool_data.push(None);
                datetime_data.push(None);
            } else {
                return Err(JsValue::from_str(
                    "Unsupported data type in WasmSeries constructor",
                ));
            }
        }
        let series = match inferred_type {
            Some(DataType::I32) => Series::new_i32(name, i32_data),
            Some(DataType::F64) => Series::new_f64(name, f64_data),
            Some(DataType::Bool) => Series::new_bool(name, bool_data),
            Some(DataType::String) => Series::new_string(name, string_data),
            Some(DataType::DateTime) => Series::new_datetime(name, datetime_data),
            None => Series::new_string(name, Vec::new()),
        };
        Ok(WasmSeries { series })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.series.name().to_string()
    }

    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        self.series.len()
    }

    #[wasm_bindgen(js_name = getValue)]
    pub fn get_value(&self, index: usize) -> JsValue {
        match self.series.get_value(index) {
            Some(Value::I32(v)) => JsValue::from_f64(v as f64),
            Some(Value::F64(v)) => JsValue::from_f64(v),
            Some(Value::Bool(v)) => JsValue::from_bool(v),
            Some(Value::String(v)) => JsValue::from_str(&v),
            Some(Value::DateTime(v)) => JsValue::from_f64(v as f64),
            Some(Value::Null) => JsValue::NULL,
            None => JsValue::UNDEFINED,
        }
    }

    #[wasm_bindgen(getter, js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    #[wasm_bindgen(getter, js_name = dataType)]
    pub fn data_type(&self) -> WasmDataType {
        self.series.data_type().into()
    }

    #[wasm_bindgen(js_name = setName)]
    pub fn set_name(&mut self, new_name: &str) {
        self.series.set_name(new_name);
    }

    #[wasm_bindgen]
    pub fn filter(&self, row_indices: Box<[usize]>) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries {
            series: self
                .series
                .filter(row_indices.as_ref())
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen]
    pub fn cast(&self, to_type: WasmDataType) -> Result<WasmSeries, JsValue> {
        let casted = self
            .series
            .cast(to_type.into())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmSeries { series: casted })
    }

    #[wasm_bindgen]
    pub fn append(&self, other: &WasmSeries) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries {
            series: self
                .series
                .append(&other.series)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.series.count()
    }

    #[wasm_bindgen]
    pub fn min(&self) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .min()
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::I32(val) => JsValue::from_f64(val as f64),
                Value::F64(val) => JsValue::from_f64(val),
                Value::DateTime(val) => JsValue::from_f64(val as f64),
                Value::String(val) => JsValue::from_str(&val),
                _ => JsValue::NULL,
            }))
    }

    #[wasm_bindgen]
    pub fn max(&self) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .max()
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::I32(val) => JsValue::from_f64(val as f64),
                Value::F64(val) => JsValue::from_f64(val),
                Value::DateTime(val) => JsValue::from_f64(val as f64),
                Value::String(val) => JsValue::from_str(&val),
                _ => JsValue::NULL,
            }))
    }

    #[wasm_bindgen]
    pub fn median(&self) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .median()
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::I32(val) => JsValue::from_f64(val as f64),
                Value::F64(val) => JsValue::from_f64(val),
                Value::DateTime(val) => JsValue::from_f64(val as f64),
                _ => JsValue::NULL,
            }))
    }

    #[wasm_bindgen(js_name = stdDev)]
    pub fn std_dev(&self) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .std_dev()
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::F64(val) => JsValue::from_f64(val),
                _ => JsValue::NULL,
            }))
    }

    #[wasm_bindgen]
    pub fn correlation(&self, other: &WasmSeries) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .correlation(&other.series)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::F64(val) => JsValue::from_f64(val),
                _ => JsValue::NULL,
            }))
    }

    #[wasm_bindgen]
    pub fn covariance(&self, other: &WasmSeries) -> Result<JsValue, JsValue> {
        Ok(self
            .series
            .covariance(&other.series)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .map_or(JsValue::NULL, |v| match v {
                Value::F64(val) => JsValue::from_f64(val),
                _ => JsValue::NULL,
            }))
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
    grouped_df: GroupedDataFrame<'static>,
}

#[wasm_bindgen]
impl WasmGroupedDataFrame {
    #[wasm_bindgen]
    pub fn agg(&self, aggregations: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let rust_aggregations: Vec<(&str, &str)> = aggregations
            .into_iter()
            .map(|js_val| {
                let arr = js_sys::Array::from(&js_val);
                let col = arr.get(0).as_string().unwrap_or_default();
                let agg = arr.get(1).as_string().unwrap_or_default();
                (
                    Box::leak(col.into_boxed_str()),
                    Box::leak(agg.into_boxed_str()),
                )
            })
            .collect();
        Ok(WasmDataFrame {
            df: self
                .grouped_df
                .agg(rust_aggregations)
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

#[wasm_bindgen]
pub struct WasmDataFrame {
    df: DataFrame,
}

#[wasm_bindgen]
impl WasmDataFrame {
    #[wasm_bindgen(constructor)]
    pub fn new(columns: &js_sys::Object) -> Result<WasmDataFrame, JsValue> {
        let mut rust_columns = BTreeMap::new();
        let entries = js_sys::Object::entries(columns);
        for entry in entries.iter() {
            let arr = js_sys::Array::from(&entry);
            let name = arr
                .get(0)
                .as_string()
                .ok_or("Column name must be a string")?;
            let js_array = js_sys::Array::from(&arr.get(1));
            let mut is_i32 = true;
            let _is_f64 = true;
            let mut is_bool = true;
            let mut is_string = true;
            let mut i32_data = Vec::new();
            let mut f64_data = Vec::new();
            let mut bool_data = Vec::new();
            let mut string_data = Vec::new();
            for v in js_array.iter() {
                if v.is_null() || v.is_undefined() {
                    i32_data.push(None);
                    f64_data.push(None);
                    bool_data.push(None);
                    string_data.push(None);
                    continue;
                }
                // Try i32
                if let Some(i) = v.as_f64() {
                    if i.fract() == 0.0 && i >= (i32::MIN as f64) && i <= (i32::MAX as f64) {
                        i32_data.push(Some(i as i32));
                    } else {
                        is_i32 = false;
                        i32_data.push(None);
                    }
                    f64_data.push(Some(i));
                } else {
                    is_i32 = false;
                    f64_data.push(None);
                }
                // Try bool
                if let Some(b) = v.as_bool() {
                    bool_data.push(Some(b));
                } else {
                    is_bool = false;
                    bool_data.push(None);
                }
                // Try string
                if let Some(s) = v.as_string() {
                    string_data.push(Some(s));
                } else {
                    is_string = false;
                    string_data.push(None);
                }
            }
            let series = if is_i32 && !i32_data.iter().all(|x| x.is_none()) {
                Series::new_i32(&name, i32_data)
            } else if is_bool && !bool_data.iter().all(|x| x.is_none()) {
                Series::new_bool(&name, bool_data)
            } else if is_string && !string_data.iter().all(|x| x.is_none()) {
                Series::new_string(&name, string_data)
            } else {
                Series::new_f64(&name, f64_data)
            };
            rust_columns.insert(name, series);
        }
        Ok(WasmDataFrame {
            df: DataFrame::new(rust_columns).map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(getter)]
    pub fn row_count(&self) -> usize {
        self.df.row_count()
    }

    #[wasm_bindgen(getter)]
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
    pub fn get_column(&self, name: &str) -> Option<WasmSeries> {
        self.df
            .get_column(name)
            .map(|s| WasmSeries { series: s.clone() })
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
        let names_vec: Vec<String> = names
            .into_iter()
            .map(|s| s.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .select_columns(names_vec)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }

    #[wasm_bindgen(js_name = dropColumns)]
    pub fn drop_columns(&self, names: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let names_vec: Vec<String> = names
            .into_iter()
            .map(|s| s.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .drop_columns(names_vec)
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
    pub fn drop_nulls(&self) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame {
            df: self
                .df
                .drop_nulls()
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

    #[wasm_bindgen(js_name = groupBy)]
    pub fn group_by(&self, by_columns: Box<[JsValue]>) -> Result<WasmGroupedDataFrame, JsValue> {
        let by_columns_vec: Vec<String> = by_columns
            .into_iter()
            .map(|s| s.as_string().unwrap_or_default())
            .collect();
        Ok(WasmGroupedDataFrame {
            grouped_df: self
                .df
                .group_by(by_columns_vec)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
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
    pub fn sort(
        &self,
        by_columns: Box<[JsValue]>,
        ascending: bool,
    ) -> Result<WasmDataFrame, JsValue> {
        let by_columns_vec: Vec<String> = by_columns
            .into_iter()
            .map(|s| s.as_string().unwrap_or_default())
            .collect();
        Ok(WasmDataFrame {
            df: self
                .df
                .sort(by_columns_vec, ascending)
                .map_err(|e| JsValue::from_str(&e.to_string()))?,
        })
    }
}
