//! Contains various structs that corresponds to objects used in this program.
//!
//! References the [official factorio wiki](https://wiki.factorio.com/Blueprint_string_format).

use serde::{Deserialize, Serialize};

/// Typedef for arbitiary inner array data structure
type InnerArray = Vec<serde_json::Value>;

#[allow(dead_code)]
pub const FACTORIO_BP_BOOK_KEY: &str = "blueprint_book";
pub const FACTORIO_BP_KEY: &str = "blueprint";
pub const FACTORIO_UP_PLANNER_KEY: &str = "upgrade_planner";
pub const FACTORIO_DECON_PLANNER_KEY: &str = "deconstruction_planner";

/// Structs defined here have a subset of attributes of their factorio equivalents.
pub mod fragments {
    use super::*;

    /// Blueprint parameters except arrays
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Blueprint {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub icons: Option<InnerArray>,
        pub item: Option<String>,
        pub label: String,
        pub label_color: Option<Color>,
        pub version: u64,
    }

    /// Blueprint book
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Book {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub icons: Option<InnerArray>,
        pub item: Option<String>,
        pub label: String,
        pub label_color: Option<Color>,
        pub active_index: u32,
        pub version: u64,
    }

    /// Con / Des planners
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Planner {
        pub label: String,
    }
}

/// Structs that are valid for use in import only.
/// Importing is converting a blueprint string to files.
pub mod importable {
    use super::*;

    /// Head of the blueprint book
    #[derive(Serialize, Deserialize, Debug)]
    pub struct BookHead {
        //used in import
        pub blueprint_book: Book,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<u16>,
    }

    /// Head of blueprint, to factorio spec
    #[derive(Serialize, Deserialize, Debug)]
    pub struct BlueprintHead {
        // used in import
        pub blueprint: Blueprint,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<u16>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpgradeHead {
        pub upgrade_planner: Planner,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<u16>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeconHead {
        pub deconstruction_planner: Planner,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<u16>,
    }
}

/// Structs that are valid for use in export only.
/// Exporting is converting a directory or a JSON file to a blueprint string.
pub mod exportable {
    use super::*;
    pub use importable::BlueprintHead as Blueprint;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BookDotFileRecursive {
        //used in export
        pub blueprint_book: BookDotFile,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub index: Option<u16>,
    }
}

/// Blueprint book with additional parameter containing the order of it's child blueprints
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    // used internally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<InnerArray>,
    pub item: Option<String>,
    pub label: String,
    pub label_color: Option<Color>,
    pub active_index: u32,
    pub version: u64,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<InnerArray>,

    /// Blueprint order in book, not part of factorio spec.
    /// Used for storing the blueprint order inside a book
    /// Contains the child blueprints/books, renamed to "order"
    #[serde(rename(serialize = "order", deserialize = "blueprints"))]
    pub order: Option<Vec<UnknownBlueprintType>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookDotFile {
    //used internally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<InnerArray>,
    pub item: Option<String>,
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,
    pub active_index: u32,
    pub version: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<InnerArray>,

    /// Blueprint order in book, not part of factorio spec.
    /// This attribute is not serialized
    #[serde(skip_serializing)]
    pub order: Option<Vec<UnknownBlueprintType>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blueprint {
    //used internally
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<InnerArray>,
    pub item: Option<String>,
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<Color>,
    pub version: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<InnerArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<InnerArray>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<InnerArray>,
}

/// Factorio's deconstruction planner / construction planner
/// both use the same top-level data structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Planner {
    pub settings: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,

    pub label: String,
    pub version: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    //used internally
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnknownBlueprintType {
    //used in common
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_book: Option<fragments::Book>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<fragments::Blueprint>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deconstruction_planner: Option<fragments::Planner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_planner: Option<fragments::Planner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u16>,
}
