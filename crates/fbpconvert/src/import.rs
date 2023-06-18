use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex};

use copypasta::{self, ClipboardContext, ClipboardProvider};
use rayon::prelude::*;

use crate::args::ImportSubCommands;
use crate::common::BlueprintType;
use crate::progress::{self, ProgressType};
use crate::{args, Execute};
use crate::{common, factorio_structs};
use factorio_structs::importable;

impl Execute for ImportSubCommands {
    fn execute(self) {
        let destination = match &self {
            ImportSubCommands::File(_f) => _f.destination.clone(),
            ImportSubCommands::Clipboard(_c) => _c.destination.clone(),
        };
        let destination = destination.unwrap_or(".".to_string());

        // create new progress tracker instance
        let mut progress_tracker = progress::Tracker::new_sync(progress::CommandType::Import);

        // make the destination dir (if it doesnt exist)
        match fs::create_dir_all(&destination) {
            Err(_) => {
                println!("Error creating deestination directory!");
                progress_tracker.lock().unwrap().complete();
                exit(1);
            }
            Ok(_) => (),
        }

        let blueprint_string: String;
        let blueprint_inflated: String;

        #[cfg(debug_assertions)]
        let inflate_only: bool;

        #[cfg(debug_assertions)]
        match &self {
            args::ImportSubCommands::File(_file) => {
                match fs::read_to_string(&_file.infile.clone().unwrap()) {
                    Ok(_str) => blueprint_string = _str,
                    Err(_) => {
                        let mut unlocked = progress_tracker.lock().unwrap();
                        unlocked.error_additional("file not found".to_string());
                        unlocked.complete();
                        exit(1);
                    }
                }
                inflate_only = _file.inflate_only;
            }

            args::ImportSubCommands::Clipboard(_copy) => {
                let mut clipboard = ClipboardContext::new().unwrap();
                match clipboard.get_contents() {
                    Ok(_clipboard) => blueprint_string = _clipboard,

                    Err(_) => {
                        let mut unlocked = progress_tracker.lock().unwrap();
                        unlocked.error_additional("clipboard empty".to_string());
                        unlocked.complete();
                        exit(1);
                    }
                }
                inflate_only = _copy.inflate_only;
            }
        }

        #[cfg(not(debug_assertions))]
        match &self.import_type {
            args::ImportSubCommands::File(_file) => {
                match fs::read_to_string(&_file.infile.clone().unwrap()) {
                    Ok(_str) => blueprint_string = _str,
                    Err(_) => {
                        let mut unlocked = progress_tracker.lock().unwrap();
                        unlocked.error_additional("file not found".to_string());
                        unlocked.complete();
                        exit(1);
                    }
                }
            }

            args::ImportSubCommands::Clipboard(_copy) => {
                let mut clipboard = ClipboardContext::new().unwrap();
                match clipboard.get_contents() {
                    Ok(_clipboard) => blueprint_string = _clipboard,

                    Err(_) => {
                        let mut unlocked = progress_tracker.lock().unwrap();
                        unlocked.error_additional("clipboard empty".to_string());
                        unlocked.complete();
                        exit(1);
                    }
                }
            }
        }

        match common::factorio_inflate(blueprint_string.as_str()) {
            Ok(blueprint) => {
                blueprint_inflated = blueprint;
            }
            Err(e) => {
                let mut unlocked = progress_tracker.lock().unwrap();
                unlocked.error_additional(e.to_string());
                unlocked.complete();
                exit(1);
            }
        }

        // convert the string to a json value
        let blueprint_obj: serde_json::Value;
        match serde_json::from_str(blueprint_inflated.as_str()) {
            Ok(_obj) => {
                blueprint_obj = _obj;
            }
            Err(_) => {
                let mut unlocked = progress_tracker.lock().unwrap();
                unlocked.error_additional(
                    "json parse error. check if blueprint string is valid".to_string(),
                );
                unlocked.complete();
                exit(1);
            }
        }

        #[cfg(debug_assertions)]
        if inflate_only {
            let mut unlocked = progress_tracker.lock().unwrap();
            unlocked.msg("inflating only...".to_string());
            let mut out_file = File::create("inflated.json").expect("file creation error");

            out_file
                .write(
                    serde_json::to_string_pretty(&blueprint_obj)
                        .expect("unable to serialize")
                        .as_bytes(),
                )
                .expect("unable to write to file");
            unlocked.complete();
            exit(0);
        }

        // let blueprint_file_name: String;
        match BlueprintType::classify(&blueprint_obj) {
            BlueprintType::Invalid => {
                let mut unlocked = progress_tracker.lock().unwrap();
                unlocked.error(
                    ProgressType::Invalid,
                    Some("invalid blueprint!".to_string()),
                );
                unlocked.complete();
                exit(1);
            }

            BlueprintType::Blueprint(_bp_name) => {
                let mut unlocked = progress_tracker.lock().unwrap();
                match blueprint_write(&blueprint_obj, &PathBuf::from(&destination)) {
                    Ok(()) => unlocked.ok(ProgressType::Blueprint(_bp_name)),
                    Err(err_msg) => {
                        unlocked.error(ProgressType::Blueprint(_bp_name), Some(err_msg))
                    }
                }
            }

            BlueprintType::Book(_book_name) => {
                match recursive_book_write(
                    &mut progress_tracker,
                    &blueprint_obj,
                    &PathBuf::from(&destination),
                ) {
                    Ok(()) => progress_tracker
                        .lock()
                        .unwrap()
                        .ok(ProgressType::Book(_book_name)),
                    Err(err_msg) => progress_tracker
                        .lock()
                        .unwrap()
                        .error(ProgressType::Book(_book_name), Some(err_msg)),
                }
            }

            BlueprintType::UpgradePlanner(_planner) => {
                match upgrade_planner_write(&blueprint_obj, &PathBuf::from(&destination)) {
                    Ok(_) => progress_tracker
                        .lock()
                        .unwrap()
                        .ok(ProgressType::UpgradePlanner(_planner)),
                    Err(err_msg) => progress_tracker
                        .lock()
                        .unwrap()
                        .error(ProgressType::Blueprint(_planner), Some(err_msg)),
                }
            }

            BlueprintType::DeconPlanner(_planner) => {
                match decon_planner_write(&blueprint_obj, &PathBuf::from(&destination)) {
                    Ok(_) => progress_tracker
                        .lock()
                        .unwrap()
                        .ok(ProgressType::DeconPlanner(_planner)),
                    Err(err_msg) => progress_tracker
                        .lock()
                        .unwrap()
                        .error(ProgressType::Blueprint(_planner), Some(err_msg)),
                }
            }
        }

        progress_tracker.lock().unwrap().complete();
    }
}


/// Writes a blueprint to file given the file path and blueprint object
/// Returns an error message if encountered
fn blueprint_write(blueprint: &serde_json::Value, dir_path: &PathBuf) -> Result<(), String> {
    let mut full_bp_path = dir_path.clone();
    let mut bp_name = blueprint
        .get(factorio_structs::FACTORIO_BP_KEY)
        .and_then(|value| value.get("label"))
        .and_then(|value| value.as_str())
        .unwrap()
        .to_string();

    // remove "index" key from the blueprint object
    let mut blueprint_compliant: importable::BlueprintHead;
    match serde_json::from_value(blueprint.to_owned()) {
        Ok(result) => blueprint_compliant = result,
        Err(_) => return Err("Error deserializing to compliant blueprint".to_string()),
    }

    bp_name = common::file_rename(bp_name);
    blueprint_compliant.blueprint.label = bp_name.clone();

    full_bp_path.push(&bp_name);
    full_bp_path.set_extension("json");

    let mut bp_file: File;

    match File::create(&full_bp_path) {
        Ok(_file) => bp_file = _file,
        Err(_) => return Err("file creation error. check the file path".to_string()),
    }

    match bp_file.write(
        serde_json::to_string_pretty(&blueprint_compliant)
            .unwrap()
            .as_bytes(),
    ) {
        Ok(_) => {
            // println!("Created {}", &full_bp_path.to_string_lossy());
            return Ok(());
        }
        Err(_) => {
            // println!("Error creating {}", &full_bp_path.to_string_lossy());
            return Err(format!(
                "Error creating {}",
                &full_bp_path.to_string_lossy()
            ));
            // return Err(bp_name);
        }
    }
}

/// Writes a upgrade planner
fn upgrade_planner_write(planner: &serde_json::Value, dir_path: &PathBuf) -> Result<(), String> {
    let mut full_planner_path = dir_path.clone();

    let mut planner_name = planner
        .get(factorio_structs::FACTORIO_UP_PLANNER_KEY)
        .and_then(|value| value.get("label"))
        .and_then(|value| value.as_str())
        .unwrap()
        .to_string();

    let mut planner_compliant: importable::UpgradeHead;
    match serde_json::from_value(planner.to_owned()) {
        Ok(result) => planner_compliant = result,
        Err(_) => return Err("Error deserializing to compliant planner".to_string()),
    }

    planner_name = common::file_rename(planner_name);
    planner_compliant.upgrade_planner.label = planner_name.clone();

    full_planner_path.push(&planner_name);
    full_planner_path.set_extension("json");

    let mut planner_file = File::create(&full_planner_path).expect("file creation error");

    match planner_file.write(
        serde_json::to_string_pretty(&planner_compliant)
            .unwrap()
            .as_bytes(),
    ) {
        Ok(_) => {
            // println!("Created {}", &full_bp_path.to_string_lossy());
            return Ok(());
        }
        Err(_) => {
            // println!("Error creating {}", &full_bp_path.to_string_lossy());
            return Err(format!(
                "Error creating {}",
                &full_planner_path.to_string_lossy()
            ));
            // return Err(bp_name);
        }
    }
}

/// Writes a decon / upgrade planner
fn decon_planner_write(planner: &serde_json::Value, dir_path: &PathBuf) -> Result<(), String> {
    let mut full_planner_path = dir_path.clone();

    let mut planner_name = planner
        .get(factorio_structs::FACTORIO_DECON_PLANNER_KEY)
        .and_then(|value| value.get("label"))
        .and_then(|value| value.as_str())
        .unwrap()
        .to_string();

    let mut planner_compliant: importable::DeconHead;
    match serde_json::from_value(planner.to_owned()) {
        Ok(result) => planner_compliant = result,
        Err(_) => return Err("Error deserializing to compliant planner".to_string()),
    }

    planner_name = common::file_rename(planner_name);
    planner_compliant.deconstruction_planner.label = planner_name.clone();

    full_planner_path.push(&planner_name);
    full_planner_path.set_extension("json");

    let mut planner_file = File::create(&full_planner_path).expect("file creation error");

    match planner_file.write(
        serde_json::to_string_pretty(&planner_compliant)
            .unwrap()
            .as_bytes(),
    ) {
        Ok(_) => {
            // println!("Created {}", &full_bp_path.to_string_lossy());
            return Ok(());
        }
        Err(_) => {
            // println!("Error creating {}", &full_bp_path.to_string_lossy());
            return Err(format!(
                "Error creating {}",
                &full_planner_path.to_string_lossy()
            ));
            // return Err(bp_name);
        }
    }
}

/// Recursively writes the book and its contents to file, given a known starting dir
/// Returns an error message if an error is encountered
fn recursive_book_write(
    prog_tracker: &mut Arc<Mutex<progress::Tracker>>,
    bp_book: &serde_json::Value,
    bp_book_dir: &PathBuf,
) -> Result<(), String> {
    // local_book_copy contains dotfile information
    let mut book_dot_file: importable::BookHead;
    match serde_json::from_value(bp_book.clone()) {
        Ok(_val) => book_dot_file = _val,
        Err(_) => return Err("failed to deserialize blueprint book".to_string()),
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
    // change iterators to rayon's parallel iterators using the for_each() method
    if let Some(ref mut _order) = book_dot_file.blueprint_book.order {
        _order.par_iter_mut().for_each(|_unknown| {
            match _unknown.blueprint.as_mut() {
                Some(_bp) => {
                    _bp.label = common::file_rename(_bp.label.clone());
                }
                None => (),
            }
            match _unknown.blueprint_book.as_mut() {
                Some(_book) => {
                    _book.label = common::file_rename(_book.label.clone());
                }
                None => (),
            }
            match _unknown.upgrade_planner.as_mut() {
                Some(_planner) => {
                    _planner.label = common::file_rename(_planner.label.clone());
                }
                None => (),
            }
            match _unknown.deconstruction_planner.as_mut() {
                Some(_planner) => {
                    _planner.label = common::file_rename(_planner.label.clone());
                }
                None => (),
            }
        });
    }

    // write the dotfile first, then constituent blueprints/books
    match fs::create_dir_all(&current_dir_path) {
        Ok(_) => (),
        Err(_) => return Err("error creating blueprint book directory".to_string()),
    }

    let mut dot_file_path = current_dir_path.clone();
    dot_file_path.push(book_dot_file_name);
    dot_file_path.set_extension("json");
    let mut dot_file: File;
    match File::create(&dot_file_path) {
        Ok(_f) => {
            dot_file = _f;
        }
        Err(_) => return Err("dotfile unable to be created".to_string()),
    }

    match dot_file.write(
        serde_json::to_string_pretty(&book_dot_file)
            .unwrap()
            .as_bytes(),
    ) {
        Ok(_) => {}
        Err(_) => return Err("error writing to dotfile".to_string()),
    }

    // get the vec of stuff
    let book_contents = bp_book
        .get("blueprint_book")
        .and_then(|value| value.get("blueprints"))
        .unwrap();

    // recurse for all constituent blueprints/books
    match book_contents {
        serde_json::Value::Array(bp_arr) => {
            bp_arr
                .par_iter()
                .for_each(|unknown_bp| match BlueprintType::classify(&unknown_bp) {
                    BlueprintType::Invalid => (),

                    BlueprintType::Book(_book_name) => {
                        match recursive_book_write(
                            &mut prog_tracker.clone(),
                            &unknown_bp,
                            &current_dir_path,
                        ) {
                            Ok(()) => prog_tracker
                                .lock()
                                .unwrap()
                                .ok(ProgressType::Book(_book_name)),
                            Err(err_msg) => prog_tracker
                                .lock()
                                .unwrap()
                                .error(ProgressType::Book(_book_name), Some(err_msg)),
                        }
                    }

                    BlueprintType::Blueprint(_bp_name) => {
                        match blueprint_write(&unknown_bp, &current_dir_path) {
                            Ok(()) => prog_tracker
                                .lock()
                                .unwrap()
                                .ok(ProgressType::Blueprint(_bp_name)),
                            Err(err_msg) => prog_tracker
                                .lock()
                                .unwrap()
                                .error(ProgressType::Blueprint(_bp_name), Some(err_msg)),
                        }
                    }
                    BlueprintType::UpgradePlanner(_planner) => {
                        match upgrade_planner_write(&unknown_bp, &current_dir_path) {
                            Ok(()) => prog_tracker
                                .lock()
                                .unwrap()
                                .ok(ProgressType::UpgradePlanner(_planner)),
                            Err(err_msg) => prog_tracker
                                .lock()
                                .unwrap()
                                .error(ProgressType::Blueprint(_planner), Some(err_msg)),
                        }
                    }
                    BlueprintType::DeconPlanner(_planner) => {
                        match decon_planner_write(&unknown_bp, &current_dir_path) {
                            Ok(()) => prog_tracker
                                .lock()
                                .unwrap()
                                .ok(ProgressType::DeconPlanner(_planner)),
                            Err(err_msg) => prog_tracker
                                .lock()
                                .unwrap()
                                .error(ProgressType::Blueprint(_planner), Some(err_msg)),
                        }
                    }
                });
        }
        _ => (),
    }

    Ok(())
}
