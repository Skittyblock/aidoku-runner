use super::wasm::env::WasmEnv;

pub fn get(env: &WasmEnv, key: u32, len: u32) -> i32 {
    if let Ok(key) = env.read_string(key, len) {
        let mut store = env.store();
        if let Some(value) = store.defaults.get(&key).cloned() {
            store.store_value(value, None)
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn set(env: &WasmEnv, key: u32, len: u32, value: i32) {
    if let Ok(key) = env.read_string(key, len) {
        let mut store = env.store();
        if let Some(value) = store.read_value(value).cloned() {
            store.defaults.insert(key, value);
        }
    }
}
