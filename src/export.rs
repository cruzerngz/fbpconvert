use std::io::Error;
use std::path::PathBuf;
use std::fs;

use serde_json::Value;

use crate::factorio_structs;

pub struct Worker {
    pub source: Option<String>,
    pub out_file: Option<String>,
    pub source_path: Option<PathBuf>
}

impl Worker {

    /// Main calling method for struct
    pub fn exec(&self) {
        todo!("Write the export module");
    }

    /// Returns the complete blueprint JSON, given a file name
    /// Returns the blueprint name if an error occurs
    fn get_blueprint(bp_file_path: &PathBuf) -> Result<Value, ()> {

        assert!(bp_file_path.is_file());
        assert_eq!(bp_file_path.extension().unwrap(), "json");

        let bp_file = fs::read_to_string(bp_file_path);

        match bp_file {
            Ok(file_contents) => {
                let json_contents: serde_json::Value = serde_json::from_str(&file_contents.as_str())
                    .expect("failed to serialize JSON data.");

                return Ok(json_contents);
            },
            Err(_) => {return Err(());}
        }

    }

    /// Recursively searches the directory to rebuild the blueprint book
    /// Returns the blueprint book name if an error occurs
    fn get_book_recursive(bp_book_dir_path: &PathBuf) -> Result<Value, String> {

        let bp_book_name = bp_book_dir_path.file_name().unwrap();
        let mut bp_book_file: PathBuf = bp_book_dir_path.clone();
        bp_book_file.push(bp_book_name);
        bp_book_file.set_extension("json");

        let mut bp_book_json: Value;
        if bp_book_file.exists() {
            bp_book_json = serde_json::from_str(
                fs::read_to_string(&bp_book_file)
                    .expect("Error reading file")
                    .as_str()
            )
                .expect("Error serializing string");
        } else {
            return Err(bp_book_name.to_str().unwrap().to_string());
        }

        // This variable contains the typed vector of UnknownBlueprintType
        // that is used to reconstruct the book
        let bp_book: factorio_structs::Book = serde_json::from_value(bp_book_json.clone()).unwrap();

        // the "order" key is consumed internally
        let bp_obj = bp_book_json.as_object_mut()
            .unwrap();

        bp_obj.remove("order");


        todo!();
    }
}
