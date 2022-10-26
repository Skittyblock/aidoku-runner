use super::wasm::env::{WasmEnv, WasmObject};
use super::wasm::models::KVC;
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use std::collections::HashMap;

// copy
pub fn copy(env: &WasmEnv, descriptor: i32) -> i32 {
    // println!("copy({})", descriptor);
    let mut store = env.store();
    if let Some(obj) = store.read_value(descriptor).cloned() {
        store.store_value(obj, None)
    } else {
        -1
    }
}
// destroy
pub fn destroy(env: &WasmEnv, obj: i32) {
    // println!("destroy({})", obj);
    env.store().remove_value(obj);
}

// create_*
pub fn create_null(env: &WasmEnv) -> i32 {
    env.store().store_value(WasmObject::Null, None)
}
pub fn create_int(env: &WasmEnv, value: i64) -> i32 {
    env.store().store_value(WasmObject::Int(value), None)
}
pub fn create_float(env: &WasmEnv, value: f64) -> i32 {
    env.store().store_value(WasmObject::Float(value), None)
}
pub fn create_bool(env: &WasmEnv, value: i32) -> i32 {
    env.store().store_value(WasmObject::Bool(value != 0), None)
}
pub fn create_string(env: &WasmEnv) -> i32 {
    env.store()
        .store_value(WasmObject::String(String::new()), None)
}
pub fn create_object(env: &WasmEnv) -> i32 {
    env.store()
        .store_value(WasmObject::Object(HashMap::new()), None)
}
pub fn create_array(env: &WasmEnv) -> i32 {
    env.store().store_value(WasmObject::Array(Vec::new()), None)
}
pub fn create_date(env: &WasmEnv, value: f64) -> i32 {
    env.store().store_value(
        WasmObject::Date(if value <= 0f64 {
            Utc::now().timestamp() as f64
        } else {
            value
        }),
        None,
    )
}

// typeof
pub fn value_kind(env: &WasmEnv, descriptor: i32) -> i32 {
    // println!("typeof({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor) {
        obj.kind()
    } else {
        WasmObject::Null.kind()
    }
}

// string_len
pub fn string_len(env: &WasmEnv, descriptor: i32) -> i32 {
    // println!("string_len({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor) {
        match obj {
            WasmObject::String(str) => str.len() as i32,
            _ => 0,
        }
    } else {
        0
    }
}
// read_*
pub fn read_string(env: &WasmEnv, descriptor: i32, buff: i32, size: i32) {
    // println!("read_string({}, {}, {})", descriptor, buff, size);
    if let Some(obj) = env.store().read_value(descriptor) {
        match obj {
            WasmObject::String(str) => {
                let size = size as usize;
                let str_buff = if size < str.len() {
                    str[..size as usize].as_bytes()
                } else {
                    str.as_bytes()
                };
                env.write_bytes(str_buff, buff as u32);
            }
            _ => (),
        };
    }
}
pub fn read_int(env: &WasmEnv, descriptor: i32) -> i64 {
    // println!("read_int({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor).cloned() {
        match obj {
            WasmObject::Int(int) => int,
            WasmObject::Float(float) => float as i64,
            WasmObject::Bool(bool) => bool as i64,
            _ => -1,
        }
    } else {
        -1
    }
}
pub fn read_float(env: &WasmEnv, descriptor: i32) -> f64 {
    if let Some(obj) = env.store().read_value(descriptor).cloned() {
        match obj {
            WasmObject::Float(float) => float,
            WasmObject::Int(int) => int as f64,
            _ => -1f64,
        }
    } else {
        -1f64
    }
}
pub fn read_bool(env: &WasmEnv, descriptor: i32) -> i32 {
    if let Some(obj) = env.store().read_value(descriptor).cloned() {
        match obj {
            WasmObject::Bool(bool) => bool as i32,
            WasmObject::Int(int) => {
                if int == 0 {
                    0
                } else {
                    1
                }
            }
            _ => 0,
        }
    } else {
        0
    }
}
pub fn read_date(env: &WasmEnv, descriptor: i32) -> f64 {
    if let Some(obj) = env.store().read_value(descriptor).cloned() {
        match obj {
            WasmObject::Date(date) => date,
            WasmObject::Float(float) => float,
            _ => -1f64,
        }
    } else {
        -1f64
    }
}

pub fn read_date_string(
    env: &WasmEnv,
    descriptor: i32,
    format: u32,
    format_len: u32,
    locale: u32,
    locale_len: u32,
    timezone: u32,
    timezone_len: u32,
) -> f64 {
    if let Some(WasmObject::String(str)) = env.store().read_value(descriptor).cloned() {
        let format = env
            .read_string(format, format_len)
            .unwrap_or_default()
            .replace("yyyy", "%Y")
            .replace("MM", "%m")
            .replace("dd", "%d")
            .replace("d", "%d")
            .replace("EEEE", "%A")
            .replace("EEE", "%a")
            .replace("HH", "%H")
            .replace("mm", "%M")
            .replace("ss", "%S");
        let _locale = env.read_string(locale, locale_len).ok();
        let _timezone = env.read_string(timezone, timezone_len).ok();
        let date = NaiveDate::parse_from_str(str.as_str(), format.as_str());
        let time = NaiveTime::parse_from_str(str.as_str(), format.as_str()).unwrap_or_default();
        if let Ok(date) = date {
            Utc.from_utc_date(&date)
                .and_time(time)
                .unwrap_or_default()
                .timestamp() as f64
        } else {
            -1f64
        }
    } else {
        -1f64
    }
}

// object_len
pub fn object_len(env: &WasmEnv, descriptor: i32) -> i32 {
    // println!("object_len({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor) {
        match obj {
            WasmObject::Object(map) => map.len() as i32,
            _ => 0,
        }
    } else {
        0
    }
}
// object_*
pub fn object_get(env: &WasmEnv, descriptor: i32, key: u32, key_len: u32) -> i32 {
    // println!("object_get({}, {}, {})", descriptor, key, key_len);
    let key = if key_len > 0 {
        env.read_string(key, key_len).unwrap_or_default()
    } else {
        String::default()
    };
    let mut store = env.store();
    if let Some(obj) = store.read_value(descriptor).cloned() {
        let value: Option<WasmObject> = match obj {
            WasmObject::Object(map) => {
                if let Some(val) = map.get(&key) {
                    Some(val.clone())
                } else {
                    None
                }
            }
            WasmObject::Filter(filter) => filter.get_value(key),
            WasmObject::Manga(manga) => manga.get_value(key),
            _ => None,
        };
        if let Some(value) = value {
            store.store_value(value, Some(descriptor))
        } else {
            -1
        }
    } else {
        -1
    }
}
pub fn object_set(env: &WasmEnv, descriptor: i32, key: u32, key_len: u32, value: i32) {
    let mut store = env.store();
    if let Ok(key) = env.read_string(key, key_len) {
        if let Some(WasmObject::Object(map)) = store.read_value(descriptor).cloned() {
            if let Some(value) = store.read_value(value).cloned() {
                let mut map = map;
                map.insert(key, value);
                store.set_value(descriptor, WasmObject::Object(map));
            }
        }
    }
}
pub fn object_values(env: &WasmEnv, descriptor: i32) -> i32 {
    let mut store = env.store();
    if let Some(obj) = store.read_value(descriptor) {
        match obj {
            WasmObject::Object(map) => {
                let arr = WasmObject::Array(map.values().cloned().collect());
                store.store_value(arr, None)
            }
            _ => -1,
        }
    } else {
        -1
    }
}

// array_len
pub fn array_len(env: &WasmEnv, descriptor: i32) -> i32 {
    // println!("array_len({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor) {
        match obj {
            WasmObject::Array(arr) => arr.len() as i32,
            _ => 0,
        }
    } else {
        0
    }
}
// array_get
pub fn array_get(env: &WasmEnv, descriptor: i32, idx: i32) -> i32 {
    // println!("array_get({}, {})", descriptor, idx);
    let mut store = env.store();
    if let Some(obj) = store.read_value(descriptor).cloned() {
        match obj {
            WasmObject::Array(arr) => {
                if let Some(value) = arr.get(idx as usize).cloned() {
                    store.store_value(value, Some(descriptor))
                } else {
                    -1
                }
            }
            _ => -1,
        }
    } else {
        -1
    }
}
// array_set
// array_append
pub fn array_append(env: &WasmEnv, descriptor: i32, value: i32) {
    // println!("array_append({}, {})", descriptor, value);
    let mut lock = env.store();
    if let Some(arr) = lock.read_value(descriptor) {
        match arr {
            WasmObject::Array(arr) => {
                if let Some(val) = lock.read_value(value) {
                    let mut arr = arr.clone();
                    arr.push(val.clone());
                    lock.set_value(descriptor, WasmObject::Array(arr));
                }
            }
            _ => (),
        }
    }
}
// array_remove
