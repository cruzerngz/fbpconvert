use std::fs;
use std::path::Path;

use serde_json::Value;

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
    /// Determines the blueprint type, returning an enum
    pub fn classify(given_bp: &Value) -> BlueprintType {
        if given_bp["blueprint_book"] != serde_json::Value::Null {
            return BlueprintType::Book(
                given_bp.get("blueprint_book")
                    .and_then(|value| value.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap()
                    .to_string()
            );
        }
        else if given_bp["blueprint"] != serde_json::Value::Null {
            return BlueprintType::Blueprint(
                given_bp.get("blueprint")
                    .and_then(|value| value.get("label"))
                    .and_then(|value| value.as_str())
                    .unwrap()
                    .to_string()
            );
        }
        else {
            return BlueprintType::Invalid;
        }
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

/// Remove any pretty-printed indentations from the json string
pub fn json_remove_indents(json_indent_str: &str) -> String {
    let json_object: serde_json::Value = serde_json::from_str(json_indent_str)
        .expect("JSON parse error. Check that blueprint string is valid.");

    return json_object.to_string();
}

