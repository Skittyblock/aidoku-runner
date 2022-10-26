use aidoku_runner::AidokuSource;
use std::slice;

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct aidoku_source_t {
    pub(crate) inner: AidokuSource,
}

#[no_mangle]
pub extern "C" fn aidoku_source_new(bytes: *const u8, len: usize) -> Option<Box<aidoku_source_t>> {
    let bytes = unsafe {
        assert!(!bytes.is_null());
        slice::from_raw_parts(bytes, len)
    };
    let source = AidokuSource::from_bytes(bytes);
    Some(Box::new(aidoku_source_t { inner: source }))
}

// just for testing
#[no_mangle]
pub extern "C" fn aidoku_source_test_manga_list(source: &aidoku_source_t) {
    if let Some(list) = source.inner.get_manga_list(Vec::new(), 1) {
        let titles = list
            .manga
            .into_iter()
            .map(|m| m.title.unwrap_or_default())
            .collect::<Vec<String>>();
        if titles.len() > 0 {
            println!("manga: {}", titles.join(", "));
        } else {
            println!("no manga found");
        }
    } else {
        println!("failed to get manga list");
    }
}
