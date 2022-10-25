use aidoku_runner::wasm::env::WasmObject;
use aidoku_runner::wasm::models::{Filter, FilterType};
use aidoku_runner::AidokuSource;

fn source() -> AidokuSource {
    let bytes = include_bytes!("main.wasm");
    let source = AidokuSource::new(bytes);
    source.initialize();
    source
}

#[test]
pub fn test_implemented() {
    let source = source();

    assert!(source
        .instance
        .exports
        .get_function("get_manga_list")
        .is_ok());
    assert!(source
        .instance
        .exports
        .get_function("get_manga_details")
        .is_ok());
    assert!(source
        .instance
        .exports
        .get_function("get_chapter_list")
        .is_ok());
    assert!(source
        .instance
        .exports
        .get_function("get_page_list")
        .is_ok());
}

// #[test]
// pub fn test_descriptor_count() {
//     let source = source();

//     let std_count = { source.env.store().std_descriptors.len() };
//     _ = source.get_manga_list(Vec::new(), 1);
//     assert_eq!(std_count, source.env.store().std_descriptors.len());

//     _ = source.get_manga_details(Manga::new(String::from("1")));
//     assert_eq!(std_count, source.env.store().std_descriptors.len());
// }

#[test]
pub fn test_manga_list() {
    let source = source();

    let filters = vec![Filter {
        kind: FilterType::Title,
        name: String::from("Title"),
        value: Box::new(WasmObject::String(String::from("1"))),
    }];

    if let Some(list) = source.get_manga_list(filters, 1) {
        let titles = list
            .manga
            .into_iter()
            .map(|m| m.title.unwrap_or_default())
            .collect::<Vec<String>>()
            .join(", ");
        println!("manga: {}", titles);
    }
}

// use std::io;
// use std::io::prelude::*;

// #[test]
// pub fn cli() {
//     println!("Aidoku CLI");

//     let bytes = include_bytes!("../main.wasm");
//     let source = AidokuSource::new(bytes);
//     source.initialize();

//     println!("Loaded Source: en.test");

//     // println!("(1) Load home page");
//     // println!("(2) Search source");

//     loop {
//         print!("> ");
//         _ = io::stdout().flush();

//         let mut input = String::new();
//         let stdin = io::stdin();
//         stdin.read_line(&mut input).unwrap();
//         input.pop(); // remove newline

//         match input.as_str() {
//             "1" | "all" => {
//                 // load manga list
//                 if let Some(list) = source.get_manga_list(Vec::new(), 1) {
//                     let titles = list
//                         .manga
//                         .into_iter()
//                         .map(|m| format!("{} ({})", m.title.unwrap_or_default(), m.id))
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     println!("Results: {:?}", titles);
//                 } else {
//                     println!("Failed to load manga list.");
//                 }
//             }
//             "2" | "search" => {
//                 // search
//                 print!("Search: ");
//                 _ = io::stdout().flush();
//                 input.clear();
//                 stdin.read_line(&mut input).unwrap();
//                 input.pop();

//                 let filters = vec![Filter {
//                     kind: FilterType::Title,
//                     name: String::from("Title"),
//                     value: Box::new(WasmObject::String(input)),
//                 }];
//                 if let Some(list) = source.get_manga_list(filters, 1) {
//                     let titles = list
//                         .manga
//                         .into_iter()
//                         .map(|m| format!("{} ({})", m.title.unwrap_or_default(), m.id))
//                         .collect::<Vec<String>>()
//                         .join(", ");
//                     println!("Results: {:?}", titles);
//                 } else {
//                     println!("Failed to load manga list.");
//                 }
//             }
//             s if s.starts_with("open ") => {
//                 let id = s[5..].to_string();
//                 if let Some(manga) = source.get_manga_details(Manga::new(id.clone())) {
//                     println!("{:?}", manga);
//                 } else {
//                     println!("Failed to load manga with id {}.", id);
//                 }
//             }
//             "q" | "quit" | "exit" => break,
//             _ => println!("Invalid option. Type 'q' to quit."),
//         };
//     }
// }
