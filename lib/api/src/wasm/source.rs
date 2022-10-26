use super::env::{WasmEnv, WasmObject};
use super::imports;
use super::models::{Chapter, Filter, Listing, Manga, MangaResult, Page};
use wasmer::{Instance, Module, Store, Value};

struct Deferred<T: FnOnce()>(Option<T>);

impl<T: FnOnce()> Drop for Deferred<T> {
    fn drop(&mut self) {
        self.0.take().map(|f| f());
    }
}

#[derive(Clone)]
pub struct AidokuSource {
    pub env: WasmEnv,
    pub store: Store,
    pub instance: Instance,
}

impl AidokuSource {
    pub fn from_bytes(module: &[u8]) -> Self {
        Self::new_with_env(module, WasmEnv::new())
    }

    pub fn new_with_env(module: &[u8], env: WasmEnv) -> Self {
        let store = Store::default();
        let module = Module::new(&store, &module).unwrap();

        let import_object = imports::generate_imports(&store, &env);
        let instance = Instance::new(&module, &import_object).unwrap();

        AidokuSource {
            env,
            store,
            instance,
        }
    }
}

impl AidokuSource {
    pub fn initialize(&self) {
        if let Ok(initialize) = self.instance.exports.get_function("initialize") {
            _ = initialize.call(&[]);
        };
    }

    pub fn get_manga_list(&self, filters: Vec<Filter>, page: i32) -> Option<MangaResult> {
        let filters_descriptor = {
            if filters.len() > 0 {
                self.env.store().store_value(
                    WasmObject::Array(filters.into_iter().map(|f| WasmObject::Filter(f)).collect()),
                    None,
                )
            } else {
                -1
            }
        };

        let _defer = Deferred(Some(|| -> () {
            if filters_descriptor != -1 {
                self.env.store().remove_value(filters_descriptor);
            }
        }));

        let get_manga_list = self.instance.exports.get_function("get_manga_list").ok()?;
        let descriptor = get_manga_list
            .call(&[Value::I32(filters_descriptor), Value::I32(page)])
            .ok()?;

        let descriptor = descriptor[0].i32()?;
        if descriptor != -1 {
            let mut store = self.env.store();
            let result = store.read_value(descriptor)?.clone();
            store.remove_value(descriptor);
            if let WasmObject::MangaResult(result) = result {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_manga_listing(&self, listing: Listing, page: i32) -> Option<MangaResult> {
        let listing_descriptor = {
            self.env
                .store()
                .store_value(WasmObject::Listing(listing), None)
        };

        let _defer = Deferred(Some(|| -> () {
            if listing_descriptor != -1 {
                self.env.store().remove_value(listing_descriptor);
            }
        }));

        let get_manga_listing = self
            .instance
            .exports
            .get_function("get_manga_listing")
            .ok()?;
        let descriptor = get_manga_listing
            .call(&[Value::I32(listing_descriptor), Value::I32(page)])
            .ok()?;

        let descriptor = descriptor[0].i32()?;
        if descriptor != -1 {
            let mut store = self.env.store();
            let result = store.read_value(descriptor)?.clone();
            store.remove_value(descriptor);
            if let WasmObject::MangaResult(result) = result {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_manga_details(&self, manga: Manga) -> Option<Manga> {
        let manga_descriptor = { self.env.store().store_value(WasmObject::Manga(manga), None) };

        let _defer = Deferred(Some(|| -> () {
            if manga_descriptor != -1 {
                self.env.store().remove_value(manga_descriptor);
            }
        }));

        let get_manga_details = self
            .instance
            .exports
            .get_function("get_manga_details")
            .ok()?;
        let descriptor = get_manga_details
            .call(&[Value::I32(manga_descriptor)])
            .ok()?;

        let descriptor = descriptor[0].i32()?;
        if descriptor != -1 {
            let mut store = self.env.store();
            let result = store.read_value(descriptor)?.clone();
            store.remove_value(descriptor);
            if let WasmObject::Manga(result) = result {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_chapter_list(&self, manga: Manga) -> Option<Vec<Chapter>> {
        let manga_descriptor = { self.env.store().store_value(WasmObject::Manga(manga), None) };

        let _defer = Deferred(Some(|| -> () {
            if manga_descriptor != -1 {
                self.env.store().remove_value(manga_descriptor);
            }
        }));

        let get_chapter_list = self
            .instance
            .exports
            .get_function("get_chapter_list")
            .ok()?;
        let descriptor = get_chapter_list
            .call(&[Value::I32(manga_descriptor)])
            .ok()?;

        let descriptor = descriptor[0].i32()?;
        if descriptor != -1 {
            let mut store = self.env.store();
            let result = store.read_value(descriptor)?.clone();
            store.remove_value(descriptor);
            if let WasmObject::Array(result) = result {
                Some(
                    result
                        .into_iter()
                        .filter_map(|c| match c {
                            WasmObject::Chapter(c) => Some(c),
                            _ => None,
                        })
                        .collect(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_page_list(&self, chapter: Chapter) -> Option<Vec<Page>> {
        let chapter_descriptor = {
            self.env
                .store()
                .store_value(WasmObject::Chapter(chapter), None)
        };

        let _defer = Deferred(Some(|| -> () {
            if chapter_descriptor != -1 {
                self.env.store().remove_value(chapter_descriptor);
            }
        }));

        let get_page_list = self.instance.exports.get_function("get_page_list").ok()?;
        let descriptor = get_page_list.call(&[Value::I32(chapter_descriptor)]).ok()?;

        let descriptor = descriptor[0].i32()?;
        if descriptor != -1 {
            let mut store = self.env.store();
            let result = store.read_value(descriptor)?.clone();
            store.remove_value(descriptor);
            if let WasmObject::Array(result) = result {
                Some(
                    result
                        .into_iter()
                        .filter_map(|p| match p {
                            WasmObject::Page(p) => Some(p),
                            _ => None,
                        })
                        .collect(),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    // pub fn get_image_request(&self, _url: &str) -> Option<> {}

    // pub fn handle_url(&self, _url: &str) -> Option<DeepLink> {}

    pub fn handle_notification(&self, notification: &str) {
        let descriptor = {
            self.env
                .store()
                .store_value(WasmObject::String(notification.to_string()), None)
        };
        let _defer = Deferred(Some(|| -> () {
            if descriptor != -1 {
                self.env.store().remove_value(descriptor);
            }
        }));
        if let Ok(handle_notification) = self.instance.exports.get_function("handle_notification") {
            _ = handle_notification.call(&[Value::I32(descriptor)]);
        }
    }
}
