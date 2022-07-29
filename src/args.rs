use clap::Parser;

pub use self::commands::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct MainCliArgs {

    #[clap(subcommand)]
    pub command: MainSubCommands,

    /// Show what the program is doing
    #[clap(short, long)]
    pub verbose: bool

}

/// This enum contains the various subcommands available
#[derive(Parser, Debug)]
pub enum MainSubCommands {

    /// Import a blueprint string
    #[clap(arg_required_else_help = true)]
    #[clap(subcommand)]
    Import(ImportSubCommands),

    /// Export a single file or JSON tree as a blueprint string
    #[clap(arg_required_else_help = true)]
    Export(ExportFile)
}

#[derive(Parser, Debug)]
pub enum ImportSubCommands {
    /// Import blueprint strings as a file
    #[clap(arg_required_else_help = true)]
    File(ImportFile),

    /// Import blueprint strings from a link
    #[clap(arg_required_else_help = true)]
    Link(ImportLink),

    /// Import blueprint strings from the clipboard
    #[clap(arg_required_else_help = true)]
    Clipboard(ImportClipboard)
}

/// Contains CLI flags/arguments for various commands/subcommands
pub mod commands {
    use super::*;

    #[derive(Parser, Debug)]
    pub struct ImportFile {
        /// Infile containing blueprint string
        #[clap(value_parser)]
        pub infile: Option<String>,

        /// Destination directory (optional)
        #[clap(short, long)]
        pub destination: Option<String>
    }

    #[derive(Parser, Debug)]
    pub struct ImportLink {
        /// URL to blueprint
        #[clap(value_parser)]
        pub link: Option<String>,

        /// Destination directory (optional)
        #[clap(short, long)]
        pub destination: Option<String>
    }

    /// Import string directly (not sure if the terminal can handle this)
    #[derive(Parser, Debug)]
    pub struct ImportClipboard {
        /// Full blueprint string
        #[clap(value_parser)]
        pub blueprint_string: Option<String>,

        /// Destination directory (optional)
        #[clap(short, long)]
        pub destination: Option<String>
    }

    #[derive(Parser, Debug)]
    pub struct ExportFile {
        /// Source directory or single JSON file
        #[clap(value_parser)]
        pub source: Option<String>,

        /// Outfile name (optional)
        #[clap(short, long)]
        pub outfile: Option<String>,

        /// Destination directory (optional)
        #[clap(short, long)]
        pub destination: Option<String>
    }
}
