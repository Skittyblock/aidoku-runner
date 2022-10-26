use crate::wasm;
use crate::wasm::env::WasmEnv;
use wasmer::{imports, Function, ImportObject, Store};

pub mod aidoku;
pub mod defaults;
pub mod env;
pub mod json;
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
            "create_null" => Function::new_native_with_env(&store, env.clone(), std::create_null),
            "create_int" => Function::new_native_with_env(&store, env.clone(), std::create_int),
            "create_float" => Function::new_native_with_env(&store, env.clone(), std::create_float),
            "create_bool" => Function::new_native_with_env(&store, env.clone(), std::create_bool),
            "create_string" => Function::new_native_with_env(&store, env.clone(), std::create_string),
            "create_object" => Function::new_native_with_env(&store, env.clone(), std::create_object),
            "create_array" => Function::new_native_with_env(&store, env.clone(), std::create_array),
            "create_date" => Function::new_native_with_env(&store, env.clone(), std::create_date),
            "string_len" => Function::new_native_with_env(&store, env.clone(), std::string_len),
            "read_string" => Function::new_native_with_env(&store, env.clone(), std::read_string),
            "read_int" => Function::new_native_with_env(&store, env.clone(), std::read_int),
            "read_float" => Function::new_native_with_env(&store, env.clone(), std::read_float),
            "read_bool" => Function::new_native_with_env(&store, env.clone(), std::read_bool),
            "read_date" => Function::new_native_with_env(&store, env.clone(), std::read_date),
            "read_date_string" => Function::new_native_with_env(&store, env.clone(), std::read_date_string),
            "object_len" => Function::new_native_with_env(&store, env.clone(), std::object_len),
            "object_get" => Function::new_native_with_env(&store, env.clone(), std::object_get),
            "object_set" => Function::new_native_with_env(&store, env.clone(), std::object_set),
            "object_values" => Function::new_native_with_env(&store, env.clone(), std::object_values),
            "array_len" => Function::new_native_with_env(&store, env.clone(), std::array_len),
            "array_get" => Function::new_native_with_env(&store, env.clone(), std::array_get),
            "array_append" => Function::new_native_with_env(&store, env.clone(), std::array_append),
         },
         "aidoku" => {
             "create_manga" => Function::new_native_with_env(&store, env.clone(), aidoku::create_manga),
             "create_manga_result" => Function::new_native_with_env(&store, env.clone(), aidoku::create_manga_result),
             "create_chapter" => Function::new_native_with_env(&store, env.clone(), aidoku::create_chapter),
             "create_page" => Function::new_native_with_env(&store, env.clone(), aidoku::create_page),
             "create_deeplink" => Function::new_native_with_env(&store, env.clone(), aidoku::create_deeplink),
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
             "json" => Function::new_native_with_env(&store, env.clone(), net::json),
         },
         "json" => {
             "parse" => Function::new_native_with_env(&store, env.clone(), json::parse),
         },
         "defaults" => {
             "get" => Function::new_native_with_env(&store, env.clone(), defaults::get),
             "set" => Function::new_native_with_env(&store, env.clone(), defaults::set),
         }
    }
}
