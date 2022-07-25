mod args;
mod import;
mod export;
mod common;
mod factorio_structs;
mod progress;

use std::process::exit;

use args::*;
use clap::Parser;


fn main() {
    let main_args = CliArgs::parse();

    match &main_args.command {
        SubCommands::Import {
            infile,
            destination,
        } => {
            match infile {
                Some(file_path) => {
                    match common::PathType::classify(&file_path.as_str()) {
                        common::PathType::Invalid => {
                            println!("File '{}' not found!", file_path);
                            exit(1);
                        },
                        _ => ()
                    }
                },
                None => {
                    println!("No file specified!");
                    exit(1)
                }
            }

            let import_args = import::Worker {
                in_file: infile.clone().unwrap(),
                dest: destination.clone().unwrap_or(".".to_string()),
                dest_path: None
            };
            import_args.exec();
        },

        SubCommands::Export {
            source,
            outfile,
            destination
        } => {

            let export_args = export::Worker {
                source: source.clone().unwrap(),
                out_file: outfile.clone(),
                dest: destination.clone()
            };

            export_args.exec();
        }

    }

}
