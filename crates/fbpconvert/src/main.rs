mod args;
mod common;
mod export;
mod factorio_structs;
mod import;
mod progress;

use args::*;
use clap::Parser;

/// Executor trait. For running commands / subcommands
pub(crate) trait Execute {
    fn execute(self);
}

impl Execute for MainCliArgs {
    fn execute(self) {
        match self.command {
            MainSubCommands::Import(import) => import.execute(),
            MainSubCommands::Export(export) => export.execute(),
        }
    }
}

fn main() {
    let main_args = MainCliArgs::parse();

    // if let Some(_gen) = main_args.generator {
    //     let cmd = MainCliArgs::command();
    //     println!("generating completion file: {_gen:?}");
    //     println!("{cmd:?}");
    // } else {
    //     println!("{main_args:#?}");
    // }

    main_args.execute()

    // old
    //
    // match &main_args.command {
    //     MainSubCommands::Import(_cmd_type) => {
    //         let import_worker = import::Worker::from(_cmd_type);
    //         import_worker.exec();
    //     }

    //     MainSubCommands::Export(_cmd_type) => {
    //         let export_worker = export::Worker::from(_cmd_type);
    //         export_worker.exec();
    //     }
    // }
}
