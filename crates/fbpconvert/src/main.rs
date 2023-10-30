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

    main_args.execute()
}
