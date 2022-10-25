use super::wasm::env::{WasmEnv, WasmObject};
use super::wasm::models::KVC;

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

// create_null
// create_*
pub fn create_array(env: &WasmEnv) -> i32 {
    // println!("create_array()");
    let vec: Vec<WasmObject> = Vec::new();
    env.store().store_value(WasmObject::Array(vec), None)
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
// read_string
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
// read_*
pub fn read_int(env: &WasmEnv, descriptor: i32) -> i64 {
    // println!("read_int({})", descriptor);
    if let Some(obj) = env.store().read_value(descriptor) {
        match obj {
            WasmObject::Int(int) => *int as i64,
            _ => -1,
        }
    } else {
        -1
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
                    lock.std_descriptors
                        .insert(descriptor, WasmObject::Array(arr));
                }
            }
            _ => (),
        }
    }
}
// array_remove
