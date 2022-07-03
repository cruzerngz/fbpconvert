//! Contains various structs that corresponds to objects used in this program.
//!
//! References the [official factorio wiki](https://wiki.factorio.com/Blueprint_string_format).

use serde::{Deserialize, Serialize};

/// Head of the blueprint book
#[derive(Serialize, Deserialize, Debug)]
pub struct BookHead {
    pub blueprint_book: Book
}

/// Head of blueprint
#[derive(Serialize, Deserialize, Debug)]
pub struct BlueprintHead {
    pub blueprint: Blueprint
}

/// Blueprint book parameters
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub item: Option<String>,
    pub label: String,
    pub label_color: Option<Color>,
    pub active_index: u32,
    pub version: u64,

    /// Blueprint order in book, not part of factorio spec.
    pub order: Option<Vec<UnknownBlueprintType>>
}

/// Blueprint parameters except arrays
#[derive(Serialize, Deserialize, Debug)]
pub struct Blueprint {
    pub item: Option<String>,
    pub label: String,
    pub label_color: Option<Color>,
    pub version: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnknownBlueprintType {
    pub blueprint_book: Option<Book>,
    pub blueprint: Option<Blueprint>
}
