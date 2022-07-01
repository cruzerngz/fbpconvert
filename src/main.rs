mod args;
mod import;
mod export;
mod common;
mod factorio_structs;

use std::{fs, process::exit};

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
            infile,
            outfile,
            source
        } => {
            println!(
                "TODO: Export JSON tree from {:?} and write blueprint string to {:?}",
                &source,
                &outfile
            );


            let file_contents = fs::read(source.as_ref().unwrap()).expect("Invalid file path!");
            let blueprint_str = common::factorio_deflate(
                std::str::from_utf8(&file_contents).unwrap()
            );

            println!("Blueprint string:\n{:?}", blueprint_str);
            let export_args = export::Worker {
                source: source.clone(),
                out_file: outfile.clone(),
                source_path: None
            };

            export_args.exec();
        }

    }

}
