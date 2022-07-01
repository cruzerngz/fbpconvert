use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {

    #[clap(subcommand)]
    pub command: SubCommands,

    /// Show what the program is doing
    #[clap(short, long)]
    pub verbose: bool

}

/// This enum contains the various subcommands available
#[derive(Parser, Debug)]
pub enum SubCommands {

    /// Import a blueprint string as a single file or a JSON tree
    #[clap(arg_required_else_help = true)]
    Import {
        /// Infile containing blueprint string
        #[clap(value_parser)]
        infile: Option<String>,

        /// Destination directory (optional), for blueprint books/single blueprints
        #[clap(short, long)]
        destination: Option<String>
    },

    /// Export a single file or JSON tree as a blueprint string
    #[clap(arg_required_else_help = true)]
    Export {
        /// Source directory or JSON file
        #[clap(value_parser)]
        source: Option<String>,

        /// Infile, for single blueprints
        #[clap(short, long)]
        infile: Option<String>,

        /// Outfile containing blueprint string
        #[clap(short, long)]
        outfile: Option<String>
    }
}
