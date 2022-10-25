use super::wasm::env::{WasmEnv, WasmObject};
use super::wasm::models::{self, Chapter, DeepLink, Manga, MangaResult, Page};

fn read_str(env: &WasmEnv, ptr: u32, len: u32) -> Option<String> {
    if len > 0 {
        env.read_string(ptr, len).ok()
    } else {
        None
    }
}

pub fn create_manga(
    env: &WasmEnv,
    id: u32,
    id_len: u32,
    cover_url: u32,
    cover_url_len: u32,
    title: u32,
    title_len: u32,
    author: u32,
    author_len: u32,
    artist: u32,
    artist_len: u32,
    description: u32,
    description_len: u32,
    url: u32,
    url_len: u32,
    tags: u32,
    tags_str_lens: u32,
    tags_len: u32,
    status: i32,
    nsfw: i32,
    viewer: i32,
) -> i32 {
    // println!("create_manga()");
    if let Some(id) = read_str(env, id, id_len) {
        let tag_descriptors: Vec<i32> = env.read_values(tags, tags_len).unwrap();
        let tag_lengths: Vec<i32> = env.read_values(tags_str_lens, tags_len).unwrap();
        let categories: Vec<String> = tag_descriptors
            .into_iter()
            .enumerate()
            .filter_map(|(idx, descriptor)| {
                env.read_string(descriptor as u32, *tag_lengths.get(idx).unwrap() as u32)
                    .ok()
            })
            .collect();

        let manga = Manga {
            id,
            cover: read_str(env, cover_url, cover_url_len),
            title: read_str(env, title, title_len),
            author: read_str(env, author, author_len),
            artist: read_str(env, artist, artist_len),
            description: read_str(env, description, description_len),
            url: read_str(env, url, url_len),
            categories,
            status: models::MangaStatus::from(status),
            nsfw: models::MangaContentRating::from(nsfw),
            viewer: models::MangaViewer::from(viewer),
        };

        env.store().store_value(WasmObject::Manga(manga), None)
    } else {
        -1
    }
}

pub fn create_manga_result(env: &WasmEnv, manga_arr: i32, has_more: i32) -> i32 {
    // println!("create_manga_result()");
    let mut store = env.store();
    if let Some(arr) = store.read_value(manga_arr) {
        match arr {
            WasmObject::Array(arr) => {
                let manga: Vec<Manga> = arr
                    .into_iter()
                    .filter_map(|o| match o {
                        WasmObject::Manga(m) => Some(m.clone()),
                        _ => None,
                    })
                    .collect();
                let result = MangaResult {
                    manga,
                    has_more: has_more == 1,
                };
                store.store_value(WasmObject::MangaResult(result), None)
            }
            _ => -1,
        }
    } else {
        -1
    }
}

pub fn create_chapter(
    env: &WasmEnv,
    id: u32,
    id_len: u32,
    title: u32,
    title_len: u32,
    volume: f32,
    chapter: f32,
    date_uploaded: f64,
    scanlator: u32,
    scanlator_len: u32,
    url: u32,
    url_len: u32,
    lang: u32,
    lang_len: u32,
) -> i32 {
    if let Some(id) = read_str(env, id, id_len) {
        let chapter = Chapter {
            id,
            title: read_str(env, title, title_len),
            volume: if volume >= 0f32 { Some(volume) } else { None },
            chapter: if chapter >= 0f32 { Some(chapter) } else { None },
            date_uploaded: if date_uploaded > 0f64 {
                Some(date_uploaded)
            } else {
                None
            },
            scanlator: read_str(env, scanlator, scanlator_len),
            url: read_str(env, url, url_len),
            lang: read_str(env, lang, lang_len),
        };
        env.store().store_value(WasmObject::Chapter(chapter), None)
    } else {
        -1
    }
}

pub fn create_page(
    env: &WasmEnv,
    index: i32,
    image_url: u32,
    image_url_len: u32,
    base64: u32,
    base64_len: u32,
    text: u32,
    text_len: u32,
) -> i32 {
    let page = Page {
        index,
        image_url: read_str(env, image_url, image_url_len),
        base64: read_str(env, base64, base64_len),
        text: read_str(env, text, text_len),
    };
    env.store().store_value(WasmObject::Page(page), None)
}

pub fn create_deeplink(env: &WasmEnv, manga: i32, chapter: i32) -> i32 {
    let store = env.store();
    let deeplink = DeepLink {
        manga: if let Some(WasmObject::Manga(manga)) = store.read_value(manga).cloned() {
            Some(manga)
        } else {
            None
        },
        chapter: if let Some(WasmObject::Chapter(chapter)) = store.read_value(chapter).cloned() {
            Some(chapter)
        } else {
            None
        },
    };
    env.store()
        .store_value(WasmObject::DeepLink(deeplink), None)
}
