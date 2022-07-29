use serde_json::Value;

use crate::factorio_structs;

pub const INVALID_CHARS: &str = r#" /\<>:"|?*"#;

/// For categorising the type of blueprint in JSON value
#[derive(Debug)]
pub enum BlueprintType {
    Invalid,
    Book(String),
    Blueprint(String)
}

impl BlueprintType {
    /// Determines the blueprint type, returning an enum with the enclosing blueprint's name
    pub fn classify(given_bp: &Value) -> BlueprintType {

        let unknown_bp_type: factorio_structs::UnknownBlueprintType;
        match serde_json::from_value(given_bp.clone()) {
            Ok(_val) => {
                unknown_bp_type = _val;
            },
            Err(e) => {
                println!("Error: {}", e);
                return BlueprintType::Invalid
            },
        }

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
            // println!("invalid character {}", &character);
            new_file_name.push('_');
        } else {
            new_file_name.push(character);
        }
    }

    return new_file_name;
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_classify_invalid_empty() {
        assert!(matches!(
            BlueprintType::classify(
                &json!({})
            ),
            BlueprintType::Invalid
        ));
    }

    #[test]
    fn test_classify_invalid_nonsense() {
        assert!(matches!(
            BlueprintType::classify(
                &json!({
                    "blueprints": {
                        "asd": "xyz"
                    }
                })
            ),
            BlueprintType::Invalid
        ));
    }

    #[test]
    fn test_classify_valid_bp() {
        assert!(matches!(
            BlueprintType::classify(
                &json!({
                    "blueprint": {
                        "item": "asd",
                        "label": "blueprint_thang",
                        "version": 1234567890
                    }
                })
            ),
            BlueprintType::Blueprint(_)
        ));
    }
    #[test]
    fn test_classify_valid_book() {
        assert!(matches!(
            BlueprintType::classify(
                &json!({
                    "blueprint_book": {
                        "item": "asd",
                        "label": "blueprint_thang",
                        "active_index": 0,
                        "version": 1234567890
                    }
                })
            ),
            BlueprintType::Book(_)
        ));
    }
}
