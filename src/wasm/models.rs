use super::env::WasmObject;

pub trait KVC {
    fn get_value(&self, key: String) -> Option<WasmObject>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MangaStatus {
    Unknown = 0,
    Ongoing = 1,
    Completed = 2,
    Cancelled = 3,
    Hiatus = 4,
}

impl From<i32> for MangaStatus {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Unknown,
            1 => Self::Ongoing,
            2 => Self::Completed,
            3 => Self::Cancelled,
            4 => Self::Hiatus,
            _ => Self::Unknown,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MangaContentRating {
    Safe = 0,
    Suggestive = 1,
    Nsfw = 2,
}

impl From<i32> for MangaContentRating {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Safe,
            1 => Self::Suggestive,
            2 => Self::Nsfw,
            _ => Self::Safe,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MangaViewer {
    Rtl = 1,
    Ltr = 2,
    Vertical = 3,
    Scroll = 4,
}

impl From<i32> for MangaViewer {
    fn from(val: i32) -> Self {
        match val {
            1 => Self::Rtl,
            2 => Self::Ltr,
            3 => Self::Vertical,
            4 => Self::Scroll,
            _ => Self::Rtl,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Manga {
    pub id: String,
    pub cover: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub artist: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub categories: Vec<String>,
    pub status: MangaStatus,
    pub nsfw: MangaContentRating,
    pub viewer: MangaViewer,
}

impl Manga {
    pub fn new(id: String) -> Self {
        Manga {
            id,
            cover: None,
            title: None,
            author: None,
            artist: None,
            description: None,
            url: None,
            categories: Vec::new(),
            status: MangaStatus::Unknown,
            nsfw: MangaContentRating::Safe,
            viewer: MangaViewer::Rtl,
        }
    }
}

impl KVC for Manga {
    fn get_value(&self, key: String) -> Option<WasmObject> {
        match key.as_str() {
            "id" => Some(WasmObject::String(self.id.clone())),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MangaResult {
    pub manga: Vec<Manga>,
    pub has_more: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FilterType {
    Base = 0,
    Group = 1,
    Text = 2,
    Check = 3,
    Select = 4,
    Sort = 5,
    SortSelection = 6,
    Title = 7,
    Author = 8,
    Genre = 9,
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub kind: FilterType,
    pub name: String,
    pub value: Box<WasmObject>,
}

impl KVC for Filter {
    fn get_value(&self, key: String) -> Option<WasmObject> {
        match key.as_str() {
            "type" => Some(WasmObject::Int(self.kind as i32)),
            "name" => Some(WasmObject::String(self.name.clone())),
            "value" => Some(*self.value.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Listing {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub volume: f32,
    pub chapter: f32,
    pub date_updated: f64,
    pub scanlator: String,
    pub url: String,
    pub lang: String,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub index: i32,
    pub url: String,
    pub base64: String,
    pub text: String,
}
