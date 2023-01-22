mod args;
mod common;
mod export;
mod factorio_structs;
mod import;
mod progress;
// mod test_constants;

use args::*;
use clap::Parser;
// use clap::CommandFactory;
// use clap_complete::{generate, Generator, Shell};

fn main() {
    let main_args = MainCliArgs::parse();

    // if let Some(_gen) = main_args.generator {
    //     let cmd = MainCliArgs::command();
    //     println!("generating completion file: {_gen:?}");
    //     println!("{cmd:?}");
    // } else {
    //     println!("{main_args:#?}");
    // }

    match &main_args.command {
        MainSubCommands::Import(_cmd_type) => {
            let import_worker = import::Worker::from(_cmd_type);
            import_worker.exec();
        }

        MainSubCommands::Export(_cmd_type) => {
            let export_worker = export::Worker::from(_cmd_type);
            export_worker.exec();
        }
    }
}
