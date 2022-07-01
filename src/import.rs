use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;

use crate::{common, factorio_structs};
use crate::common::{BlueprintType, PathType};

pub struct Worker {
    pub in_file: String,
    pub dest: String,
    pub dest_path: Option<PathBuf>
}

impl Worker {

    /// Main calling method for struct
    pub fn exec(&self) {
        // make the destination dir (if it doesnt exist)
        match fs::create_dir_all(&self.dest) {
            Err(_) => {
                println!("Error creating deestination directory!");
                exit(1);
            },
            Ok(_) => ()
        }

        let blueprint_string = fs::read_to_string(&self.in_file).unwrap();
        let blueprint_inflated:String;

        match common::factorio_inflate(blueprint_string.as_str()) {
            Ok(blueprint) => {
                blueprint_inflated = blueprint;
            },
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }

        // convert the string to a json value
        let blueprint_obj: serde_json::Value = serde_json::from_str(blueprint_inflated.as_str())
            .expect("JSON parse error. Check that the blueprint string is valid.");

        let blueprint_file_name: String;
        match BlueprintType::classify(&blueprint_obj) {
            BlueprintType::Invalid => {
                println!("Invalid blueprint!");
                exit(1);
            }
            BlueprintType::Blueprint(bp_name) => {
                match Worker::blueprint_write(&blueprint_obj, &PathBuf::from(&self.dest)) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            }
            BlueprintType::Book(book_name) => {

                match Worker::recursive_book_write(&blueprint_obj, &PathBuf::from(&self.dest)) {
                    Ok(_) => {},
                    Err(_) => {}
                }

                // blueprint_file_name = book_name;
            }
        }

    }

    /// Writes a blueprint to file given the file path and blueprint object
    fn blueprint_write(blueprint: &serde_json::Value, dir_path: &PathBuf) -> Result<(), ()> {
        let mut full_bp_path = dir_path.clone();
        let bp_name = blueprint.get("blueprint")
            .and_then(|value| value.get("label"))
            .and_then(|value| value.as_str())
            .unwrap()
            .to_string();

        full_bp_path.push(bp_name);
        full_bp_path.set_extension("json");

        let mut bp_file = File::create(&full_bp_path)
            .expect("File creation error. Check the file path given");

        match bp_file.write(
            serde_json::to_string_pretty(&blueprint)
                .unwrap()
                .as_bytes()
        ) {
            Ok(_) => {
                println!("Created {}", &full_bp_path.to_string_lossy());
                return Ok(());
            },
            Err(_) => {
                println!("Error creating {}", &full_bp_path.to_string_lossy());
                return Err(());
            }
        }
    }

    /// Recursively writes the book and its contents to file, given a known starting dir
    fn recursive_book_write(bp_book: &serde_json::Value, dir_path: &PathBuf) -> Result<(), ()> {

        // println!("Blueprint book full: {:?}", bp_book);

        // Extract out only the relavant blueprint contents
        let book_details: factorio_structs::BookHead = serde_json::from_value(bp_book.clone())
            .expect("Failed to deserialize blueprint book");

        // println!("Blueprint book contents: {:?}", &book_details);

        // create new starting dir (for next recursion level)
        let mut new_starting_dir = dir_path.clone();
        new_starting_dir.push(&book_details.blueprint_book.label);
        match fs::create_dir_all(&new_starting_dir) {
            Ok(_) => {
                println!("Created dir {}", &new_starting_dir.to_string_lossy());
            },
            Err(_) => {
                println!("Error creating dir {}", &new_starting_dir.to_string_lossy());
            }
        }

        // write blueprint book details inside starting dir
        // blueprint book details are stored in a dotfile that matches the dir name
        let mut bp_book_path = dir_path.clone();
        let mut bp_book_name = ".".to_string();
        bp_book_name.push_str(&book_details.blueprint_book.label);
        bp_book_path.push(&book_details.blueprint_book.label);
        bp_book_path.push(bp_book_name);
        bp_book_path.set_extension("json");

        let mut bp_book_file = File::create(&bp_book_path)
        .expect("File creation error.");

        match bp_book_file.write(
            serde_json::to_string_pretty(&book_details)
            .unwrap()
            .as_bytes()
        ) {
            Ok(_) => {
                println!("Created {}", &bp_book_path.to_string_lossy());
            }
            Err(_) => {
                println!("Error creating {}", &bp_book_path.to_string_lossy());
            }
        }

        // get the arr of blueprints
        let book_contents = bp_book.get("blueprint_book")
            .and_then(|value| value.get("blueprints"))
            .unwrap();

        // recurse
        match book_contents {
            serde_json::Value::Array(bp_arr) => {
                for bp_arr_item in bp_arr.iter() {
                    match BlueprintType::classify(bp_arr_item) {
                        BlueprintType::Invalid => (), // ignore

                        BlueprintType::Book(_) => {
                            // perform recursive write
                            match Worker::recursive_book_write(bp_arr_item, &new_starting_dir) {
                                Ok(_) => (),
                                Err(_) => {
                                    println!("Book write error: {}", &book_details.blueprint_book.label);
                                }
                            }
                        },

                        BlueprintType::Blueprint(sub_bp_name) => {
                            match Worker::blueprint_write(bp_arr_item, &new_starting_dir) {
                                Ok(_) => (),
                                Err(_) => {
                                    println!("Blueprint write error: {}", &sub_bp_name);
                                }
                            }
                        }
                    }
                }
            }
            _ => ()
        }

        Ok(())
    }
}


