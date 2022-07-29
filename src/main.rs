mod args;
mod import;
mod export;
mod common;
mod factorio_structs;
mod progress;

use args::*;
use clap::Parser;


fn main() {
    let main_args = MainCliArgs::parse();

    match &main_args.command {
        MainSubCommands::Import(_cmd_type) => {
            let import_worker = import::Worker::from(_cmd_type);
            import_worker.exec();
        },

        MainSubCommands::Export(_cmd_type) => {
            let export_worker = export::Worker::from(_cmd_type);
            export_worker.exec();
        }

    }
}
