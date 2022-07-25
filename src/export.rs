use std::path::PathBuf;
use std::fs;
use std::process::exit;

use serde_json::Value;

use crate::factorio_structs::exportable;
use crate::common;
use crate::progress::{self, ProgressType};

/// Prefix for exported blueprints
const PREFIX_OUT: &str = "fbpconvert-bp_";

pub struct Worker {
    pub source: String,
    pub out_file: Option<String>,
    // pub source_path: Option<PathBuf>
}

impl Worker {

    /// Main calling method for struct
    pub fn exec(&self) {
        let source_path = PathBuf::from(&self.source);

        let mut progress_tracker = progress::Tracker::new(progress::CommandType::Export);
        let mut read_json_value =  serde_json::json!({});

        match source_path.extension() {
            Some(ext) => { // read single file
                if ext.eq_ignore_ascii_case("json") {
                    match Worker::read_blueprint(&source_path) {
                        Ok(json_object) => {
                            read_json_value = json_object;
                            progress_tracker.ok(
                                ProgressType::Blueprint(
                                    source_path
                                        .to_str()
                                        .unwrap()
                                        .to_string()
                                )
                            )
                        },
                        Err(err_msg) => {
                            progress_tracker.error(
                                ProgressType::Blueprint(source_path
                                    .to_str()
                                    .unwrap()
                                    .to_string()
                                ),
                                Some(err_msg)
                            )
                        }
                    }
                } else {
                    progress_tracker.error_additional("file given is not JSON!".to_string());
                    progress_tracker.complete();
                    exit(1);
                }
            },

            None => { // read blueprint book (recursive)
                match Worker::read_book_recursive(&mut progress_tracker, &source_path) {
                    Ok(json_object) => {
                        read_json_value = json_object;
                    },
                    Err(err_msg) => {
                        progress_tracker.error(
                            ProgressType::Book(source_path
                                .to_str()
                                .unwrap()
                                .to_string()),
                            Some(err_msg)
                        )
                    }
                }
            }
        }
        // println!("final constructed: {:?}", &read_json_value);

        match self.write_blueprint_to_file(&read_json_value) {
            Ok(()) => (),
            Err(err_msg) => {
                progress_tracker.error_additional(err_msg);
            }
        }
        progress_tracker.complete();
    }

    /// Returns the complete blueprint JSON, given a file name
    /// Returns an error message if an error occurs
    fn read_blueprint(bp_file_path: &PathBuf) -> Result<Value, String> {

        if !bp_file_path.is_file() {
            println!("{:?}", bp_file_path);
            return Err("not a file".to_string());
        }
        match bp_file_path.extension() {
            None => {
                return Err("no file extension".to_string())
            }
            Some(file_ext) => {
                if !file_ext.eq_ignore_ascii_case("json") {
                    return Err("wrong file extension".to_string())
                }
            }
        }

        assert!(bp_file_path.is_file());
        assert_eq!(bp_file_path.extension().unwrap(), "json");

        let bp_file = fs::read_to_string(bp_file_path);

        match bp_file {
            Ok(file_contents) => {
                let json_contents: serde_json::Value;
                match serde_json::from_str(&file_contents.as_str()) {
                    Ok(_contents) => json_contents = _contents,
                    Err(_) => return Err("failed to serialize blueprint data".to_string()),
                }

                return Ok(json_contents);
            },
            Err(_) => {return Err(
                bp_file_path
                    .to_str()
                    .unwrap()
                    .to_string()
                );
            }
        }

    }

    /// Recursively searches the directory to rebuild the blueprint book
    /// Returns an error message if an error occurs
    fn read_book_recursive(
        prog_tracker: &mut progress::Tracker,
        bp_book_dir_path: &PathBuf
    ) -> Result<Value, String> {
        // println!("reading: {:?}", &bp_book_dir_path);

        let book_name = bp_book_dir_path.file_name().unwrap().to_str().unwrap();
        let mut dot_file_path = bp_book_dir_path.clone();
        let current_dir_path = bp_book_dir_path.clone(); // used in recursion
        // set the dotfile path
        dot_file_path.push(format!(".{}.json", book_name));

        let dot_file_contents: String;
        match fs::read_to_string(&dot_file_path) {
            Ok(_file) => dot_file_contents = _file,
            Err(_) => return Err("failed to read file".to_string())
        }

        let mut book_object: exportable::BookDotFileHead;

        match serde_json::from_str(dot_file_contents.as_ref()) {
            Ok(_book) => book_object = _book,
            Err(_) => return Err("failed to deserialize contents".to_string())
        }

        book_object.blueprint_book.blueprints = Some(vec![]);

        // println!("{:#?}", book_object);

        // iterate through the list of stored blueprints
        match &book_object.blueprint_book.order {
            Some(unknown_bps) => {
                for unknown_blueprint in unknown_bps.iter() {

                    match &unknown_blueprint.blueprint {
                        Some(known_bp) => {

                            let mut known_bp_path = current_dir_path.clone();
                            known_bp_path.push(&known_bp.label);
                            known_bp_path.set_extension("json");

                            let known_bp_object: Option<Value>;
                            match Worker::read_blueprint(&known_bp_path) {
                                Ok(_bp_obj) => {
                                    known_bp_object = Some(_bp_obj);
                                    prog_tracker.ok(
                                        ProgressType::Blueprint(
                                            known_bp_path.file_name()
                                                .unwrap()
                                                .to_str()
                                                .unwrap()
                                                .to_string()
                                        )
                                    )
                                },
                                Err(err_msg) => {
                                    known_bp_object = None;
                                    prog_tracker.error(
                                        ProgressType::Blueprint(
                                            known_bp_path.file_name()
                                                .unwrap()
                                                .to_str()
                                                .unwrap()
                                                .to_string()
                                        ),
                                        Some(err_msg)
                                    )
                                }
                            }

                            match known_bp_object {
                                Some(_bp_obj) => {
                                    if let Some(blueprint_vec) = &mut book_object.blueprint_book.blueprints {
                                        blueprint_vec.push(_bp_obj);
                                    }
                                },
                                None => ()
                            }

                        },
                        None => ()
                    }
                    match &unknown_blueprint.blueprint_book {
                        Some(known_book) => {

                            let mut known_book_path = current_dir_path.clone();
                            known_book_path.push(&known_book.label);

                            let known_book_object: Option<Value>;
                            match Worker::read_book_recursive(prog_tracker, &known_book_path) {
                                Ok(_book_obj) => {
                                    known_book_object = Some(_book_obj);
                                    prog_tracker.ok(ProgressType::Book(
                                        known_book_path.file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string()
                                        )
                                    )
                                },
                                Err(err_msg) => {
                                    known_book_object = None;
                                    prog_tracker.error(ProgressType::Book(
                                        known_book_path.file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_string()
                                        ),
                                        Some(err_msg)
                                    );
                                }
                            }

                            match known_book_object {
                                Some(_book_obj) => {
                                    if let Some(blueprint_vec) = &mut book_object.blueprint_book.blueprints {
                                        blueprint_vec.push(_book_obj);
                                    }
                                },
                                None => ()
                            }
                        },
                        None => ()
                    }
                }
            }
            None => ()
        }

        match serde_json::to_value(book_object) {
            Ok(_val) => {
                Ok(_val)
            },
            Err(_) => {
                Err("failed to convert typed struct to serde_json::Value".to_string())
            }
        }

    }

    /// Takes the blueprint and writes it to a destination
    /// Returns an error message if it occurs
    pub fn write_blueprint_to_file(
        &self,
        blueprint_json: &Value,
    ) -> Result<(),String> {

        let mut destination: String;
        match &self.out_file {
            Some(dest) => destination = dest.to_owned(),
            None => {
                match common::BlueprintType::classify(&blueprint_json) {
                    common::BlueprintType::Invalid => {
                        return Err("failed to determine blueprint type".to_string())
                    },
                    common::BlueprintType::Blueprint(name) => {
                        destination = name;
                    },
                    common::BlueprintType::Book(name) => {
                        destination = name;
                    }
                }
                destination = format!("{}{}",PREFIX_OUT, destination);
            }
        }

        // println!("dest: {}", destination);

        match serde_json::to_string(blueprint_json) {
            Ok(blueprint_string) => {
                let blueprint_string_deflated = common::factorio_deflate(blueprint_string.as_ref());
                match fs::write(destination, blueprint_string_deflated.as_bytes()) {
                    Ok(_) => {return Ok(())},
                    Err(_) => {
                        return Err("file write error".to_string());
                    }
                }
            },
            Err(_) => return Err("serde_json serialize error".to_string())
        }

    }
}
