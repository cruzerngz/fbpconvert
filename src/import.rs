use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;

use crate::factorio_structs::UnknownBlueprintType;
use crate::{common, factorio_structs};
use crate::common::{BlueprintType, PathType};
use crate::progress::{self, ProgressType};

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

        // create new progress tracker instance
        let mut progress_tracker = progress::Tracker::new(progress::CommandType::Import);

        // convert the string to a json value
        let blueprint_obj: serde_json::Value = serde_json::from_str(blueprint_inflated.as_str())
            .expect("JSON parse error. Check that the blueprint string is valid.");

        // let blueprint_file_name: String;
        match BlueprintType::classify(&blueprint_obj) {
            BlueprintType::Invalid => {
                println!("Invalid blueprint!");
                exit(1);
            }
            BlueprintType::Blueprint(_bp_name) => {
                match Worker::blueprint_write(&blueprint_obj, &PathBuf::from(&self.dest)) {
                    Ok(()) => progress_tracker.ok(ProgressType::Blueprint(_bp_name)),
                    Err(err_msg) => progress_tracker.error(ProgressType::Blueprint(_bp_name), Some(err_msg))
                }
            }
            BlueprintType::Book(_book_name) => {
                match Worker::recursive_book_write(&mut progress_tracker, &blueprint_obj, &PathBuf::from(&self.dest)) {
                    Ok(()) => progress_tracker.ok(ProgressType::Book(_book_name)),
                    Err(err_msg) => progress_tracker.error(ProgressType::Book(_book_name), Some(err_msg))
                }
            }
        }

        progress_tracker.complete();

    }

    /// Writes a blueprint to file given the file path and blueprint object
    /// Returns an error message if encountered
    fn blueprint_write(
        blueprint: &serde_json::Value,
        dir_path: &PathBuf
    ) -> Result<(), String> {
        let mut full_bp_path = dir_path.clone();
        let mut bp_name = blueprint.get("blueprint")
            .and_then(|value| value.get("label"))
            .and_then(|value| value.as_str())
            .unwrap()
            .to_string();

        // remove "index" key from the blueprint object
        let blueprint_compliant: factorio_structs::BlueprintHead;
        match serde_json::from_value(blueprint.to_owned()) {
            Ok(result) => blueprint_compliant = result,
            Err(_) => return Err("Error deserializing to compliant blueprint".to_string())
        }

        bp_name = common::file_rename(bp_name);

        full_bp_path.push(&bp_name);
        full_bp_path.set_extension("json");

        let mut bp_file = File::create(&full_bp_path)
            .expect("File creation error. Check the file path given");

        match bp_file.write(
            serde_json::to_string_pretty(&blueprint_compliant)
                .unwrap()
                .as_bytes()
        ) {
            Ok(_) => {
                // println!("Created {}", &full_bp_path.to_string_lossy());
                return Ok(());
            },
            Err(_) => {
                // println!("Error creating {}", &full_bp_path.to_string_lossy());
                return Err(format!("Error creating {}", &full_bp_path.to_string_lossy()));
                // return Err(bp_name);
            }
        }

    }

    /// Recursively writes the book and its contents to file, given a known starting dir
    /// Returns an error message if an error is encountered
    fn recursive_book_write(
        prog_tracker: &mut progress::Tracker,
        bp_book: &serde_json::Value,
        dir_path: &PathBuf
    ) -> Result<(), String> {

        // println!("Blueprint book full: {:?}", bp_book);

        // Extract out only the relavant blueprint contents
        let mut book_details: factorio_structs::BookHead = serde_json::from_value(bp_book.clone())
            .expect("Failed to deserialize blueprint book");

        let bp_book_name = book_details.blueprint_book.label.clone();

        // println!("Blueprint book contents: {:?}", &book_details);

        // create new starting dir (for next recursion level)
        let mut new_starting_dir = dir_path.clone();
        new_starting_dir.push(&book_details.blueprint_book.label);
        match fs::create_dir_all(&new_starting_dir) {
            Ok(_) => {
                // println!("Created dir {}", &new_starting_dir.to_string_lossy());
            },
            Err(_) => {
                // println!("Error creating dir {}", &new_starting_dir.to_string_lossy());
                return Err(format!("error creating dir {}", &new_starting_dir.to_string_lossy()));
            }
        }

        // write blueprint book details inside starting dir
        // blueprint book details are stored in a dotfile that matches the dir name
        let mut bp_book_path = dir_path.clone();
        let mut bp_book_name = ".".to_string();

        bp_book_name = common::file_rename(bp_book_name);
        bp_book_name.push_str(&book_details.blueprint_book.label);
        bp_book_path.push(&book_details.blueprint_book.label);
        bp_book_path.push(&bp_book_name);
        bp_book_path.set_extension("json");

        let mut bp_book_file = File::create(&bp_book_path)
        .expect("File creation error.");

        match bp_book_file.write(
            serde_json::to_string_pretty(&book_details)
            .unwrap()
            .as_bytes()
        ) {
            Ok(_) => {
                // println!("Created {}", &bp_book_path.to_string_lossy());
            }
            Err(_) => {
                // println!("Error creating {}", &bp_book_path.to_string_lossy());
                return Err(format!("error creating {}", &bp_book_path.to_string_lossy()));
            }
        }

        book_details.blueprint_book.order;

        // get the arr of blueprints
        let book_contents = bp_book.get("blueprint_book")
            .and_then(|value| value.get("blueprints"))
            .unwrap();

        book_details.blueprint_book.order = Some(vec!());

        // recurse
        match book_contents {
            serde_json::Value::Array(bp_arr) => {
                for bp_arr_item in bp_arr.iter() {
                    // store the order of blueprint book items
                    // bp_book_order =
                    // then recurse
                    match BlueprintType::classify(bp_arr_item) {
                        BlueprintType::Invalid => (), // ignore

                        BlueprintType::Book(_book_name) => {
                            // set order to none in child blueprints
                            // bp_arr_item.get("order")
                                // .and_then(|val| val.get());

                            // perform recursive write
                            match Worker::recursive_book_write(prog_tracker, bp_arr_item, &new_starting_dir) {
                                Ok(()) => prog_tracker.ok(ProgressType::Book(_book_name)),
                                Err(err_msg) => prog_tracker.error(ProgressType::Book(_book_name), Some(err_msg))
                            }
                        },

                        BlueprintType::Blueprint(_bp_name) => {
                            match Worker::blueprint_write(bp_arr_item, &new_starting_dir) {
                                Ok(()) => prog_tracker.ok(ProgressType::Blueprint(_bp_name)),
                                Err(err_msg) => prog_tracker.error(ProgressType::Blueprint(_bp_name), Some(err_msg))
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


