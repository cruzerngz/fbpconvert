use serde::{Deserialize, Serialize};

/// Head of the blueprint book
#[derive(Serialize, Deserialize, Debug)]
pub struct BookHead {
    pub blueprint_book: Book
}

/// Contains blueprint book parameters
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub item: String,
    pub label: String,
    pub label_color: Option<Color>,
    pub active_index: u32,
    pub version: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}
