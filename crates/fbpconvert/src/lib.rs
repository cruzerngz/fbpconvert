pub use args::{MainCliArgs, MainSubCommands};
use clap::CommandFactory;
use clap_complete::generator::generate;
use clap_complete::shells;
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
            MainSubCommands::Generate(complete) => {
                let mut app: clap::Command = MainCliArgs::command();
                let mut fd = std::io::stdout();

                let bin = match self.bin_name {
                    Some(b) => b,
                    None => {
                        println!("Executable name not found. Is this running as a library?");
                        std::process::exit(-1)
                    }
                };

                #[rustfmt::skip]
                match complete.completions {
                    args::ShellCli::Bash => generate(shells::Bash, &mut app, bin, &mut fd),
                    args::ShellCli::Zsh => generate(shells::Zsh, &mut app, bin, &mut fd),
                    args::ShellCli::Fish => generate(shells::Fish, &mut app, bin, &mut fd),
                    args::ShellCli::PowerShell => generate(shells::PowerShell, &mut app, bin, &mut fd),
                    args::ShellCli::Elvish => generate(shells::Elvish, &mut app, bin, &mut fd),
                };
            }
        }
    }
}
