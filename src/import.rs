use std::fs;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;

use copypasta::{self, ClipboardContext, ClipboardProvider};
// use reqwest::blocking::Response;

use crate::{common, factorio_structs};
use factorio_structs::importable;
use crate::common::BlueprintType;
use crate::progress::{self, ProgressType};
use crate::args;

pub struct Worker {
    pub import_type: args::ImportSubCommands,
    dest: String
}

impl Worker {

    pub fn from(_cmd_type: &args::ImportSubCommands) -> Worker {
        Worker {
            import_type: _cmd_type.clone(),
            dest: match &_cmd_type {
                args::ImportSubCommands::File(_file) => {
                    _file.destination.clone().unwrap_or(".".to_string())
                },
                // args::ImportSubCommands::Link(_link) => {
                //     _link.destination.clone().unwrap_or(".".to_string())
                // },
                args::ImportSubCommands::Clipboard(_copy) => {
                    _copy.destination.clone().unwrap_or(".".to_string())
                }
            }
        }
    }

    /// Main calling method for struct
    pub fn exec(&self) {

        // create new progress tracker instance
        let mut progress_tracker = progress::Tracker::new(progress::CommandType::Import);

        // make the destination dir (if it doesnt exist)
        match fs::create_dir_all(&self.dest) {
            Err(_) => {
                println!("Error creating deestination directory!");
                progress_tracker.complete();
                exit(1);
            },
            Ok(_) => ()
        }

        let blueprint_string: String;
        let blueprint_inflated:String;

        match &self.import_type {
            args::ImportSubCommands::File(_file) => {
                match fs::read_to_string(&_file.infile.clone().unwrap()) {
                    Ok(_str) => blueprint_string = _str,
                    Err(_) => {
                        progress_tracker.error_additional("file not found".to_string());
                        progress_tracker.complete();
                        exit(1);
                    },
                }
            },
            // args::ImportSubCommands::Link(_link) => {
            //     let mut resp: Response;
            //     match &_link.link {
            //         Some(_link) => {
            //             match reqwest::blocking::get(_link) {
            //                 Ok(_resp) => {
            //                     resp = _resp;
            //                 },
            //                 Err(_) => {
            //                     progress_tracker.error_additional("invalid link".to_string());
            //                     progress_tracker.complete();
            //                     exit(1);
            //                 },
            //             }
            //         },
            //         None => {
            //             progress_tracker.error_additional("no link given".to_string());
            //             progress_tracker.complete();
            //             exit(1);
            //         },
            //     }

            //     match resp.read_to_string(&mut blueprint_string) {
            //         Ok(_size) => {
            //             progress_tracker.msg_temp(format!("{} bytes downloaded", _size))
            //         },
            //         Err(_) => {
            //             progress_tracker.error_additional("unable to read to string".to_string());
            //             progress_tracker.complete();
            //             exit(1);
            //         },
            //     }

            //     // println!("Import link command not impl'd yet!");
            //     // progress_tracker.complete();
            //     // exit(1);
            // },
            args::ImportSubCommands::Clipboard(_copy) => {
                let mut clipboard = ClipboardContext::new().unwrap();
                match clipboard.get_contents() {
                    Ok(_clipboard) => blueprint_string = _clipboard,

                    Err(_) => {
                        progress_tracker.error_additional("clipboard empty".to_string());
                        progress_tracker.complete();
                        exit(1);
                    },
                }
            },
        }

        match common::factorio_inflate(blueprint_string.as_str()) {
            Ok(blueprint) => {
                blueprint_inflated = blueprint;
            },
            Err(e) => {
                progress_tracker.error_additional(e.to_string());
                progress_tracker.complete();
                exit(1);
            }
        }

        // convert the string to a json value
        let blueprint_obj: serde_json::Value;
        match serde_json::from_str(blueprint_inflated.as_str()) {
            Ok(_obj) => {
                blueprint_obj = _obj;
            },
            Err(_) => {
                progress_tracker.error_additional(
                    "json parse error. check if blueprint string is valid".to_string()
                );
                progress_tracker.complete();
                exit(1);
            },
        }

        // let blueprint_file_name: String;
        match BlueprintType::classify(&blueprint_obj) {
            BlueprintType::Invalid => {
                println!("Invalid blueprint!");
                progress_tracker.complete();
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
        let mut blueprint_compliant: importable::BlueprintHead;
        match serde_json::from_value(blueprint.to_owned()) {
            Ok(result) => blueprint_compliant = result,
            Err(_) => return Err("Error deserializing to compliant blueprint".to_string())
        }

        bp_name = common::file_rename(bp_name);
        blueprint_compliant.blueprint.label = bp_name.clone();

        full_bp_path.push(&bp_name);
        full_bp_path.set_extension("json");

        let mut bp_file: File;

        match File::create(&full_bp_path) {
            Ok(_file) => {
                bp_file = _file
            },
            Err(_) => {
                return Err("file creation error. check the file path".to_string())
            },
        }

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
        bp_book_dir: &PathBuf
    ) -> Result<(), String> {

        // local_book_copy contains dotfile information
        let mut book_dot_file: importable::BookHead;
        match serde_json::from_value(bp_book.clone()) {
            Ok(_val) => {
                book_dot_file = _val
            },
            Err(_) => return Err("failed to deserialize blueprint book".to_string())
        }

        // remove invalid characters from book by renaming
        book_dot_file.blueprint_book.label = common::file_rename(book_dot_file.blueprint_book.label);
        // book dotfile name, resides in book directory
        let mut book_dot_file_name = ".".to_string();
        book_dot_file_name.push_str(&book_dot_file.blueprint_book.label);

        // new starting dir for next recursion level
        let mut current_dir_path = bp_book_dir.clone();
        current_dir_path.push(&book_dot_file.blueprint_book.label);

        // iterator for the contents of dotfile book
        // rename all names in dotfile (remove invalid chars)
        if let Some(ref mut _order) = book_dot_file.blueprint_book.order {
            for _unknown in _order.iter_mut() {
                match _unknown.blueprint.as_mut() {
                    Some(_bp) => {
                        _bp.label = common::file_rename(_bp.label.clone());
                    },
                    None => (),
                }
                match _unknown.blueprint_book.as_mut() {
                    Some(_book) => {
                        _book.label = common::file_rename(_book.label.clone());
                    },
                    None => (),
                }
            }
        }

        // write the dotfile first, then constituent blueprints/books
        match fs::create_dir_all(&current_dir_path) {
            Ok(_) => (),
            Err(_) => return Err("error creating blueprint book directory".to_string())
        }

        let mut dot_file_path = current_dir_path.clone();
        dot_file_path.push(book_dot_file_name);
        dot_file_path.set_extension("json");
        let mut dot_file: File;
        match File::create(&dot_file_path) {
            Ok(_f) => {
                dot_file = _f;
            },
            Err(_) => return Err("dotfile unable to be created".to_string())
        }

        match dot_file.write(
            serde_json::to_string_pretty(&book_dot_file)
            .unwrap().
            as_bytes()
        ) {
            Ok(_) => {},
            Err(_) => return Err("error writing to dotfile".to_string()),
        }

        // get the vec of stuff
        let book_contents = bp_book.get("blueprint_book")
            .and_then(|value| value.get("blueprints"))
            .unwrap();

        // recurse for all constituent blueprints/books
        match book_contents {
            serde_json::Value::Array(bp_arr) => {
                for unknown_bp in bp_arr.iter() {
                    match BlueprintType::classify(&unknown_bp) {

                        BlueprintType::Invalid => (),

                        BlueprintType::Book(_book_name) => {

                            match Worker::recursive_book_write(prog_tracker, unknown_bp, &current_dir_path) {
                                Ok(()) => prog_tracker.ok(ProgressType::Book(_book_name)),
                                Err(err_msg) => prog_tracker.error(ProgressType::Book(_book_name), Some(err_msg))
                            }
                        },

                        BlueprintType::Blueprint(_bp_name) => {
                            match Worker::blueprint_write(unknown_bp, &current_dir_path) {
                                Ok(()) => prog_tracker.ok(ProgressType::Blueprint(_bp_name)),
                                Err(err_msg) => prog_tracker.error(ProgressType::Blueprint(_bp_name), Some(err_msg))
                            }
                        },
                    }
                }
            }
            _ => ()
        }

        Ok(())
    }
}


