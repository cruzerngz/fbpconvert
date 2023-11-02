pub use args::{MainCliArgs, MainSubCommands};

mod args;
mod common;
mod export;
mod factorio_structs;
mod import;
mod progress;

/// Executor trait. For running commands / subcommands
pub trait Execute {
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
