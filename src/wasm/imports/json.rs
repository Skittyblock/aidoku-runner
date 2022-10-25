use super::wasm::env::{WasmEnv, WasmGlobalStore, WasmObject};
use serde_json::Value;
use std::collections::HashMap;

pub fn parse(env: &WasmEnv, data: u32, len: u32) -> i32 {
    if len <= 0 {
        return -1;
    }
    if let Ok(str) = env.read_string(data, len) {
        parse_str(&mut env.store(), &str)
    } else {
        -1
    }
}

pub fn parse_str(store: &mut WasmGlobalStore, str: &str) -> i32 {
    let value: Value = serde_json::from_str(str).unwrap();
    let obj = parse_value(&value);
    store.store_value(obj, None)
}

fn parse_value(value: &Value) -> WasmObject {
    match value {
        arr if arr.is_array() => WasmObject::Array(
            arr.as_array()
                .unwrap()
                .into_iter()
                .map(|v| parse_value(v))
                .collect(),
        ),
        obj if obj.is_object() => WasmObject::Object(
            obj.as_object()
                .unwrap()
                .into_iter()
                .map(|m| (m.0.clone(), parse_value(m.1)))
                .collect::<HashMap<String, WasmObject>>(),
        ),
        int if int.is_i64() => WasmObject::Int(int.as_i64().unwrap()),
        float if float.is_f64() => WasmObject::Float(float.as_f64().unwrap()),
        str if str.is_string() => WasmObject::String(str.as_str().unwrap().to_string()),
        bool if bool.is_boolean() => WasmObject::Bool(bool.as_bool().unwrap()),
        null if null.is_null() => WasmObject::Null,
        _ => WasmObject::Null,
    }
}
