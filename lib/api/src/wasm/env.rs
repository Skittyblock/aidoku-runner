// use crate::{MangaObject, MangaResult};
use super::models::{Chapter, DeepLink, Filter, Listing, Manga, MangaResult, Page, KVC};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use wasmer::{LazyInit, Memory, ValueType, WasmPtr, WasmerEnv};

#[derive(Clone, Debug)]
pub enum WasmObject {
    Null,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<WasmObject>),
    Object(HashMap<String, WasmObject>),
    Date(f64),
    Node,
    Unknown,

    Manga(Manga),
    MangaResult(MangaResult),
    Filter(Filter),
    Listing(Listing),
    Chapter(Chapter),
    Page(Page),
    DeepLink(DeepLink),
}

impl WasmObject {
    pub fn kind(&self) -> i32 {
        match *self {
            Self::Null => 0,
            Self::Int(_) => 1,
            Self::Float(_) => 2,
            Self::String(_) => 3,
            Self::Bool(_) => 4,
            Self::Array(_) => 5,
            Self::Object(_) => 6,
            Self::Date(_) => 7,
            Self::Node => 8,
            Self::Unknown => 9,
            _ => 6,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HttpMethod {
    Get = 0,
    Post = 1,
    Head = 2,
    Put = 3,
    Delete = 4,
}

impl From<i32> for HttpMethod {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Get,
            1 => Self::Post,
            2 => Self::Head,
            3 => Self::Put,
            4 => Self::Delete,
            _ => Self::Get,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status_code: i32,
    pub data: Vec<u8>,
}

impl KVC for Response {
    fn get_value(&self, key: String) -> Option<WasmObject> {
        match key.as_str() {
            "status_code" => Some(WasmObject::Int(self.status_code as i64)),
            "data" => Some(WasmObject::Array(
                self.data
                    .clone()
                    .into_iter()
                    .map(|x| WasmObject::Int(x as i64))
                    .collect(),
            )),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub url: Option<String>,
    pub headers: HashMap<String, Option<String>>,
    pub body: Option<Vec<u8>>,
    pub response: Option<Response>,
}

pub struct WasmGlobalStore {
    std_pointer: i32,
    std_descriptors: HashMap<i32, WasmObject>,
    request_pointer: i32,
    requests: HashMap<i32, Request>,
    pub defaults: HashMap<String, WasmObject>,
}

impl WasmGlobalStore {
    pub fn new() -> WasmGlobalStore {
        WasmGlobalStore {
            std_pointer: -1,
            std_descriptors: HashMap::new(),
            request_pointer: -1,
            requests: HashMap::new(),
            defaults: HashMap::new(),
        }
    }

    pub fn read_value(&self, descriptor: i32) -> Option<&WasmObject> {
        self.std_descriptors.get(&descriptor)
    }

    pub fn store_value(&mut self, obj: WasmObject, _from: Option<i32>) -> i32 {
        self.std_pointer += 1;
        self.std_descriptors.insert(self.std_pointer, obj);
        self.std_pointer
    }

    pub fn set_value(&mut self, descriptor: i32, obj: WasmObject) {
        self.std_descriptors.insert(descriptor, obj);
    }

    pub fn remove_value(&mut self, descriptor: i32) {
        self.std_descriptors.remove(&descriptor);
    }
}

impl WasmGlobalStore {
    pub fn new_request(&mut self, method: HttpMethod) -> i32 {
        let request = Request {
            method,
            url: None,
            headers: HashMap::new(),
            body: None,
            response: None,
        };
        self.request_pointer += 1;
        self.requests.insert(self.request_pointer, request);
        self.request_pointer
    }

    pub fn get_request(&self, descriptor: &i32) -> Option<&Request> {
        self.requests.get(descriptor)
    }

    pub fn set_request(&mut self, descriptor: i32, request: Request) {
        self.requests.insert(descriptor, request);
    }

    pub fn remove_request(&mut self, descriptor: i32) {
        self.requests.remove(&descriptor);
    }
}

#[derive(WasmerEnv, Clone)]
pub struct WasmEnv {
    #[wasmer(export)]
    pub memory: LazyInit<Memory>,
    pub store: Arc<Mutex<WasmGlobalStore>>,
}

impl WasmEnv {
    pub fn new() -> Self {
        Self {
            memory: Default::default(),
            store: Arc::new(Mutex::new(WasmGlobalStore::new())),
        }
    }

    pub fn store(&self) -> MutexGuard<WasmGlobalStore> {
        self.store.lock().unwrap()
    }

    pub fn memory(&self) -> &Memory {
        self.memory_ref().unwrap()
    }

    pub fn read_string(&self, ptr: u32, len: u32) -> Result<String, ()> {
        let input: Vec<u8> = self.read_bytes(ptr, len)?;
        Ok(String::from_utf8_lossy(&input).to_string())
    }

    pub fn read_bytes(&self, ptr: u32, len: u32) -> Result<Vec<u8>, ()> {
        let offset: WasmPtr<u8, wasmer::Array> = WasmPtr::new(ptr);
        if let Some(buf) = offset.deref(self.memory(), 0, len) {
            Ok(buf.iter().map(|b| b.get()).collect())
        } else {
            Err(())
        }
    }

    pub fn read_values<T>(&self, ptr: u32, len: u32) -> Result<Vec<T>, ()>
    where
        T: ValueType,
    {
        let offset: WasmPtr<T, wasmer::Array> = WasmPtr::new(ptr);
        if let Some(buf) = offset.deref(self.memory(), 0, len) {
            Ok(buf.iter().map(|b| b.get()).collect())
        } else {
            Err(())
        }
    }

    pub fn write_string(&self, str: &str, offset: u32) {
        self.write_bytes(str.as_bytes(), offset);
    }

    pub fn write_bytes(&self, value: &[u8], offset: u32) {
        let view = match self.memory.get_ref() {
            Some(mem) => mem.view::<u8>(),
            _ => return,
        };
        let from = offset as usize;
        for (bytes, cell) in value.into_iter().zip(view[from..from + value.len()].iter()) {
            cell.set(*bytes);
        }
    }
}
