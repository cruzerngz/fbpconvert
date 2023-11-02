use clap::Parser;
use fbpcore::{MainCliArgs, Execute};

fn main() {
    let main_args = MainCliArgs::parse();
    main_args.execute();
}
