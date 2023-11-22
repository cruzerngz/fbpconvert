use clap::Parser;
use fbpcore::{MainCliArgs, Execute};

fn main() {
    let mut main_args = MainCliArgs::parse();
    main_args.bin_name = Some(env!("CARGO_BIN_NAME"));

    main_args.execute();
}
