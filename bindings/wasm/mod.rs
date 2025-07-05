use wasm_bindgen::prelude::*;
use crate::dataframe::DataFrame;
use crate::series::Series;
use crate::types::{DataType, Value};
use std::collections::BTreeMap;

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
            let name = arr.get(0).as_string().ok_or("Column name must be a string")?;
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
        Ok(WasmDataFrame { df: DataFrame::new(rust_columns).map_err(|e| JsValue::from_str(&e.to_string()))? })
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
        self.df.column_names().into_iter().map(|s| JsValue::from_str(s)).collect::<Vec<JsValue>>().into_boxed_slice()
    }

    #[wasm_bindgen(js_name = getColumn)]
    pub fn get_column(&self, name: &str) -> Option<WasmSeries> {
        self.df.get_column(name).map(|s| WasmSeries { series: s.clone() })
    }

    #[wasm_bindgen]
    pub fn filter(&self, row_indices: Box<[usize]>) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame { df: self.df.filter_by_indices(row_indices.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = selectColumns)]
    pub fn select_columns(&self, names: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let names_vec: Vec<String> = names.into_iter().map(|s| s.as_string().unwrap_or_default()).collect();
        Ok(WasmDataFrame { df: self.df.select_columns(names_vec).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = dropColumns)]
    pub fn drop_columns(&self, names: Box<[JsValue]>) -> Result<WasmDataFrame, JsValue> {
        let names_vec: Vec<String> = names.into_iter().map(|s| s.as_string().unwrap_or_default()).collect();
        Ok(WasmDataFrame { df: self.df.drop_columns(names_vec).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = renameColumn)]
    pub fn rename_column(&self, old_name: &str, new_name: &str) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame { df: self.df.rename_column(old_name, new_name).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = dropNulls)]
    pub fn drop_nulls(&self) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame { df: self.df.drop_nulls().map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = fillNulls)]
    pub fn fill_nulls(&self, value: WasmValue) -> Result<WasmDataFrame, JsValue> {
        Ok(WasmDataFrame { df: self.df.fill_nulls(value.value).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen]
    pub fn sort(&self, by_columns: Box<[JsValue]>, ascending: bool) -> Result<WasmDataFrame, JsValue> {
        let by_columns_vec: Vec<String> = by_columns.into_iter().map(|s| s.as_string().unwrap_or_default()).collect();
        Ok(WasmDataFrame { df: self.df.sort(by_columns_vec, ascending).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }
}

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
                return Err(JsValue::from_str("Unsupported data type in WasmSeries constructor"));
            }
        }

        let series = match inferred_type {
            Some(DataType::I32) => Series::new_i32(name, i32_data),
            Some(DataType::F64) => Series::new_f64(name, f64_data),
            Some(DataType::Bool) => Series::new_bool(name, bool_data),
            Some(DataType::String) => Series::new_string(name, string_data),
            Some(DataType::DateTime) => Series::new_datetime(name, datetime_data),
            None => Series::new_string(name, Vec::new()), // Default to empty string series if no type inferred
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

    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.series.is_empty()
    }

    #[wasm_bindgen(js_name = dataType)]
    pub fn data_type(&self) -> WasmDataType {
        self.series.data_type().into()
    }

    #[wasm_bindgen(js_name = setName)]
    pub fn set_name(&mut self, new_name: &str) {
        self.series.set_name(new_name);
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

    #[wasm_bindgen]
    pub fn filter(&self, row_indices: Box<[usize]>) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.filter(row_indices.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = fillNulls)]
    pub fn fill_nulls(&self, value: WasmValue) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.fill_nulls(&value.value).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen]
    pub fn cast(&self, to_type: WasmDataType) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.cast(to_type.into()).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen]
    pub fn append(&self, other: &WasmSeries) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.append(&other.series).map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen]
    pub fn sum(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.sum().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::I32(val) => JsValue::from_f64(val as f64),
            Value::F64(val) => JsValue::from_f64(val),
            Value::DateTime(val) => JsValue::from_f64(val as f64),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn count(&self) -> usize {
        self.series.count()
    }

    #[wasm_bindgen]
    pub fn min(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.min().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::I32(val) => JsValue::from_f64(val as f64),
            Value::F64(val) => JsValue::from_f64(val),
            Value::DateTime(val) => JsValue::from_f64(val as f64),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn max(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.max().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::I32(val) => JsValue::from_f64(val as f64),
            Value::F64(val) => JsValue::from_f64(val),
            Value::DateTime(val) => JsValue::from_f64(val as f64),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn mean(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.mean().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::F64(val) => JsValue::from_f64(val),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn median(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.median().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::I32(val) => JsValue::from_f64(val as f64),
            Value::F64(val) => JsValue::from_f64(val),
            Value::DateTime(val) => JsValue::from_f64(val as f64),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen(js_name = stdDev)]
    pub fn std_dev(&self) -> Result<JsValue, JsValue> {
        Ok(self.series.std_dev().map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::F64(val) => JsValue::from_f64(val),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn correlation(&self, other: &WasmSeries) -> Result<JsValue, JsValue> {
        Ok(self.series.correlation(&other.series).map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::F64(val) => JsValue::from_f64(val),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn covariance(&self, other: &WasmSeries) -> Result<JsValue, JsValue> {
        Ok(self.series.covariance(&other.series).map_err(|e| JsValue::from_str(&e.to_string()))?.map_or(JsValue::NULL, |v| match v {
            Value::F64(val) => JsValue::from_f64(val),
            _ => JsValue::NULL,
        }))
    }

    #[wasm_bindgen]
    pub fn unique(&self) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.unique().map_err(|e| JsValue::from_str(&e.to_string()))? })
    }

    #[wasm_bindgen(js_name = toVecF64)]
    pub fn to_vec_f64(&self) -> Result<Box<[JsValue]>, JsValue> {
        let vec_f64 = self.series.to_vec_f64().map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(vec_f64.into_iter().map(JsValue::from_f64).collect::<Vec<JsValue>>().into_boxed_slice())
    }

    #[wasm_bindgen(js_name = interpolateNulls)]
    pub fn interpolate_nulls(&self) -> Result<WasmSeries, JsValue> {
        Ok(WasmSeries { series: self.series.interpolate_nulls().map_err(|e| JsValue::from_str(&e.to_string()))? })
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
    pub fn new(val: JsValue) -> Result<WasmValue, JsValue> {
        if val.as_f64().is_some() {
            Ok(WasmValue { value: Value::F64(val.as_f64().unwrap()) })
        } else if val.as_string().is_some() {
            Ok(WasmValue { value: Value::String(val.as_string().unwrap()) })
        } else if val.as_bool().is_some() {
            Ok(WasmValue { value: Value::Bool(val.as_bool().unwrap()) })
        } else if val.is_null() || val.is_undefined() {
            Ok(WasmValue { value: Value::Null })
        } else {
            Err(JsValue::from_str("Unsupported JsValue type for WasmValue"))
        }
    }
}

impl From<Value> for WasmValue {
    fn from(value: Value) -> Self {
        WasmValue { value }
    }
}

impl From<WasmValue> for Value {
    fn from(wasm_value: WasmValue) -> Self {
        wasm_value.value
    }
}

impl From<WasmSeries> for Series {
    fn from(wasm_series: WasmSeries) -> Self {
        wasm_series.series
    }
}

impl From<Series> for WasmSeries {
    fn from(series: Series) -> Self {
        WasmSeries { series }
    }
}