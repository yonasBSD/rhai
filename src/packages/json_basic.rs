#![cfg(all(not(feature = "no_json"), feature = "json"))]
//! Rhai JSON parsing and serialization package.

#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
use crate::{def_package, plugin::*, Map};

def_package! {
    /// Package of JSON serialization/deserialization utilities.
    pub BasicJsonPackage(lib) {
        lib.set_standard_lib(true);
        combine_with_exported_module!(lib, "json", json_functions);
    }
}

#[export_module]
mod json_functions {
    use crate::{Array, Dynamic, EvalAltResult};
    use serde_json::Value;

    /// Parse a JSON string into a Rhai object (Map/Array/Value).
    ///
    /// ```rhai
    /// let data = parse_json("{\"key\": 42}");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn parse_json(s: &str) -> Result<Dynamic, Box<EvalAltResult>> {
        let value: Value = serde_json::from_str(s).map_err(|e| {
            Box::new(EvalAltResult::ErrorSystem(
                format!("Failed to parse JSON: {s}"), // Use s to show context
                format!("{e}").into(),
            ))
        })?;
        json_to_dynamic(value)
    }

    /// Serialize a Rhai object (Map/Array/Value) into a pretty-printed JSON string.
    ///
    /// ```rhai
    /// let json_string = to_json(data);
    /// ```
    #[rhai_fn(return_raw)]
    pub fn to_json(value: Dynamic) -> Result<String, Box<EvalAltResult>> {
        let json = dynamic_to_json(value)?;
        serde_json::to_string_pretty(&json)
            .map_err(|e| format!("Failed to serialize JSON: {e}").into())
    }

    /// Convert `serde_json::Value` to Rhai Dynamic
    fn json_to_dynamic(value: Value) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(match value {
            Value::Null => Dynamic::UNIT,
            Value::Bool(b) => Dynamic::from(b),
            Value::Number(n) => n
                .as_i64()
                .map(Dynamic::from)
                .or_else(|| n.as_f64().map(Dynamic::from))
                .unwrap_or(Dynamic::UNIT),
            Value::String(s) => Dynamic::from(s),
            Value::Array(arr) => {
                let vec: Result<Array, _> = arr.into_iter().map(json_to_dynamic).collect();
                Dynamic::from(vec?)
            }
            Value::Object(obj) => {
                let mut map = Map::new();
                for (k, v) in obj {
                    map.insert(k.into(), json_to_dynamic(v)?);
                }
                Dynamic::from(map)
            }
        })
    }

    /// Convert Rhai Dynamic to `serde_json::Value`
    fn dynamic_to_json(value: Dynamic) -> Result<Value, Box<EvalAltResult>> {
        if value.is_unit() {
            return Ok(Value::Null);
        }
        if let Some(b) = value.clone().try_cast::<bool>() {
            return Ok(Value::Bool(b));
        }
        if let Some(i) = value.clone().try_cast::<i64>() {
            return Ok(Value::Number(i.into()));
        }
        if let Some(f) = value.clone().try_cast::<f64>() {
            return Ok(serde_json::Number::from_f64(f).map_or(Value::Null, Value::Number));
        }
        if let Some(s) = value.clone().try_cast::<String>() {
            return Ok(Value::String(s));
        }
        if let Some(arr) = value.clone().try_cast::<Array>() {
            let vec: Result<Vec<_>, _> = arr.into_iter().map(dynamic_to_json).collect();
            return Ok(Value::Array(vec?));
        }
        if let Some(map) = value.try_cast::<Map>() {
            let obj: Result<serde_json::Map<String, Value>, _> = map
                .into_iter()
                .map(|(k, v)| dynamic_to_json(v).map(|v| (k.to_string(), v)))
                .collect();
            return Ok(Value::Object(obj?));
        }

        Err("Cannot convert value to JSON".into())
    }
}
