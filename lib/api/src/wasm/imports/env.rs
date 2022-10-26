use super::wasm::env::WasmEnv;

pub fn abort(_msg: i32, _file_name: i32, _line: i32, _column: i32) {
    println!("abort called!");
}

pub fn print(env: &WasmEnv, ptr: u32, len: u32) {
    let str = env.read_string(ptr, len);
    if let Ok(str) = str {
        println!("print: {}", str);
    } else {
        println!("print: Failed to read string for printing.");
    }
}
