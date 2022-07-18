use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::factorio_structs;

pub const INVALID_CHARS: &str = r#"/\<>:"|?*"#;

/// Various types of paths the program may encounter
#[derive(Debug)]
pub enum PathType {
    Invalid,
    File(String),
    Dir(String)
}

/// For categorising the type of blueprint in JSON value
#[derive(Debug)]
pub enum BlueprintType {
    Invalid,
    Book(String),
    Blueprint(String)
}

impl PathType {
    /// Determines the path type, returning an enum
    pub fn classify(given_path: &str) -> PathType {
        if !Path::new(given_path).exists() {
            return PathType::Invalid;
        } else {
            if Path::new(given_path).is_file() {
                return PathType::File(given_path.to_string());
            }
            else if Path::new(given_path).is_dir() {
                return PathType::Dir(given_path.to_string());
            }
            else {
                return PathType::Invalid;
            }
        }
    }
}

impl BlueprintType {
    /// Determines the blueprint type, returning an enum with the enclosing blueprint's name
    pub fn classify(given_bp: &Value) -> BlueprintType {

        let unknown_bp_type:factorio_structs::UnknownBlueprintType = serde_json::from_value(given_bp.clone())
            .expect("failed to serialize unknown blueprint type");

        match unknown_bp_type.blueprint_book {
            Some(bp_book) => {
                return BlueprintType::Book(bp_book.label);
            }
            None => ()
        }

        match unknown_bp_type.blueprint {
            Some(bp) => {
                return BlueprintType::Blueprint(bp.label);
            }
            None => ()
        }

        return BlueprintType::Invalid;
    }
}

/// Inflate the blueprint string according to factorio spec
pub fn factorio_inflate(bp_string: &str) -> Result<String, &str>{
    // skip first byte, then base64 decode
    let decoded = base64::decode(bp_string[1..].as_bytes());

    let pre_inflate;
    match decoded {
        Err(_) => return Err(&"Base64 decode error!"),
        _ => {pre_inflate = decoded.unwrap();}
    }

    let inflated = inflate::inflate_bytes_zlib(&pre_inflate);

    match inflated {
        Err(_) => {
            return Err(&"zlib inflate error!");
        },
        _ => ()
    }

    return Ok(
        std::str::from_utf8(&inflated.unwrap())
            .unwrap().to_string()
    )
}

/// Deflate the blueprint string according to factorio spec
pub fn factorio_deflate(bp_string_json: &str) -> String {
    // compress string
    let deflated = deflate::deflate_bytes_zlib(bp_string_json.as_bytes());
    let encoded = base64::encode(&deflated);

    // append a 0
    let mut result = "0".to_string();
    result.push_str(&encoded);

    return result;
}

/// Replaces all invalid characters in file names with underscores
pub fn file_rename(file_name: String) -> String {
    let mut new_file_name: String = String::new();

    for character in file_name.chars() {
        if INVALID_CHARS.contains(character) {
            println!("invalid character {}", &character);
            new_file_name.push('_');
        } else {
            new_file_name.push(character);
        }
    }

    return new_file_name;
}

/// Remove any pretty-printed indentations from the json string
pub fn json_remove_indents(json_indent_str: &str) -> String {
    let json_object: serde_json::Value = serde_json::from_str(json_indent_str)
        .expect("JSON parse error. Check that blueprint string is valid.");

    return json_object.to_string();
}

