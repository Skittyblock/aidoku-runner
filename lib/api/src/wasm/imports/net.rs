use super::json;
use super::wasm::env::{HttpMethod, Response, WasmEnv};
use bytes::Bytes;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

pub fn init(env: &WasmEnv, method: i32) -> i32 {
    env.store().new_request(HttpMethod::from(method))
}

pub fn close(env: &WasmEnv, descriptor: i32) {
    env.store().remove_request(descriptor)
}

pub fn send(env: &WasmEnv, descriptor: i32) {
    let mut store = env.store();
    let mut req = match store.get_request(&descriptor) {
        Some(x) => x.clone(),
        _ => return,
    };
    if let Some(url) = req.url.clone() {
        let res = {
            let client = reqwest::blocking::Client::new();
            let mut headers = HeaderMap::new();
            req.headers.clone().into_iter().for_each(|m| {
                if let Ok(name) = HeaderName::from_str(&m.0) {
                    if let Ok(value) = HeaderValue::from_str(&m.1.unwrap_or_default()) {
                        headers.insert(name, value);
                    }
                }
            });
            let mut builder = match req.method {
                HttpMethod::Get => client.get(url),
                HttpMethod::Post => client.post(url),
                HttpMethod::Head => client.head(url),
                HttpMethod::Put => client.put(url),
                HttpMethod::Delete => client.delete(url),
            }
            .headers(headers);
            if let Some(body) = req.body.clone() {
                builder = builder.body(body);
            }
            builder.send()
        };
        let response = if let Ok(res) = res {
            Response {
                status_code: res.status().as_u16() as i32,
                data: res.bytes().unwrap_or(Bytes::new()).to_vec(),
            }
        } else {
            Response {
                status_code: 400,
                data: Vec::new(),
            }
        };
        req.response = Some(response);
        store.set_request(descriptor, req);
    }
}

pub fn set_url(env: &WasmEnv, descriptor: i32, value: u32, len: u32) {
    if let Ok(url) = env.read_string(value, len) {
        let mut store = env.store();
        let mut req = match store.get_request(&descriptor) {
            Some(x) => x.clone(),
            _ => return,
        };
        req.url = Some(url);
        store.set_request(descriptor, req);
    }
}

pub fn set_header(
    env: &WasmEnv,
    descriptor: i32,
    key: u32,
    key_len: u32,
    value: u32,
    value_len: u32,
) {
    if let Ok(key) = env.read_string(key, key_len) {
        if let Ok(value) = env.read_string(value, value_len) {
            let mut store = env.store();
            let mut req = match store.get_request(&descriptor) {
                Some(x) => x.clone(),
                _ => return,
            };
            req.headers.insert(key, Some(value));
            store.set_request(descriptor, req);
        }
    }
}

pub fn set_body(env: &WasmEnv, descriptor: i32, value: u32, len: u32) {
    if let Ok(data) = env.read_bytes(value, len) {
        let mut store = env.store();
        let mut req = match store.get_request(&descriptor) {
            Some(x) => x.clone(),
            _ => return,
        };
        req.body = Some(data);
        store.set_request(descriptor, req);
    }
}

pub fn get_data_size(env: &WasmEnv, descriptor: i32) -> i32 {
    let store = env.store();
    let req = match store.get_request(&descriptor) {
        Some(x) => x,
        _ => return -1,
    };
    if let Some(res) = req.response.clone() {
        res.data.len() as i32
    } else {
        -1
    }
}

pub fn get_data(env: &WasmEnv, descriptor: i32, buff: i32, size: i32) {
    // println!("get_data({}, {}, {})", descriptor, buff, size);
    if let Some(req) = env.store().get_request(&descriptor) {
        if let Some(res) = req.response.clone() {
            let size = size as usize;
            let data_buff = if size < res.data.len() {
                &res.data[..size as usize]
            } else {
                res.data.as_slice()
            };
            env.write_bytes(data_buff, buff as u32);
        }
    }
}

pub fn json(env: &WasmEnv, descriptor: i32) -> i32 {
    let mut store = env.store();
    if let Some(req) = store.get_request(&descriptor) {
        if let Some(res) = req.response.clone() {
            json::parse_str(&mut store, &String::from_utf8_lossy(&res.data))
        } else {
            -1
        }
    } else {
        -1
    }
}
