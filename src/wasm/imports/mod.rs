use crate::wasm;
use crate::wasm::env::WasmEnv;
use wasmer::{imports, Function, ImportObject, Store};

pub mod aidoku;
pub mod env;
pub mod net;
pub mod std;

pub fn generate_imports(store: &Store, env: &WasmEnv) -> ImportObject {
    imports! {
        "env" => {
            "abort" => Function::new_native(&store, env::abort),
            "print" => Function::new_native_with_env(&store, env.clone(), env::print),
        },
        "std" => {
            "copy" => Function::new_native_with_env(&store, env.clone(), std::copy),
            "destroy" => Function::new_native_with_env(&store, env.clone(), std::destroy),
            "typeof" => Function::new_native_with_env(&store, env.clone(), std::value_kind),
            "create_array" => Function::new_native_with_env(&store, env.clone(), std::create_array),
            "string_len" => Function::new_native_with_env(&store, env.clone(), std::string_len),
            "read_string" => Function::new_native_with_env(&store, env.clone(), std::read_string),
            "read_int" => Function::new_native_with_env(&store, env.clone(), std::read_int),
            "object_len" => Function::new_native_with_env(&store, env.clone(), std::object_len),
            "object_get" => Function::new_native_with_env(&store, env.clone(), std::object_get),
            "array_len" => Function::new_native_with_env(&store, env.clone(), std::array_len),
            "array_get" => Function::new_native_with_env(&store, env.clone(), std::array_get),
            "array_append" => Function::new_native_with_env(&store, env.clone(), std::array_append),
         },
         "aidoku" => {
             "create_manga" => Function::new_native_with_env(&store, env.clone(), aidoku::create_manga),
             "create_manga_result" => Function::new_native_with_env(&store, env.clone(), aidoku::create_manga_result),
         },
         "net" => {
             "init" => Function::new_native_with_env(&store, env.clone(), net::init),
             "close" => Function::new_native_with_env(&store, env.clone(), net::close),
             "send" => Function::new_native_with_env(&store, env.clone(), net::send),
             "set_url" => Function::new_native_with_env(&store, env.clone(), net::set_url),
             "set_header" => Function::new_native_with_env(&store, env.clone(), net::set_header),
             "set_body" => Function::new_native_with_env(&store, env.clone(), net::set_body),
             "get_data" => Function::new_native_with_env(&store, env.clone(), net::get_data),
             "get_data_size" => Function::new_native_with_env(&store, env.clone(), net::get_data_size),
         }
    }
}
