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
    let main_args = MainCliArgs::parse();

    match &main_args.command {
        MainSubCommands::Import(import_command) => {
            match import_command {
                ImportSubCommands::File(_import_file) => {
                    let import_worker = import::Worker::from(&import_command);
                    import_worker.exec();

                },
                ImportSubCommands::Link(_import_link) => {
                    println!("Import link is a work in progress!");
                    exit(1);
                },
                ImportSubCommands::Clipboard(_import_string) => {
                    println!("Import clipboard is a work in progress!");
                    exit(1);
                },
            }
        },

        MainSubCommands::Export(export_command) => {
            let export_worker = export::Worker::from(export_command);
            export_worker.exec();
        }

    }

}
