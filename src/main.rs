mod args;
mod import;
mod export;
mod common;
mod factorio_structs;
mod progress;
// mod test_constants;

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

#[cfg(notset)]
mod test {
    use super::*;
    use serde_json::{Value, json};

    use test_constants::constants::FACTORIO_BP_STRING as SAMPLE_BP;

    #[test]
    fn test_equal_values() {
        let reference = json!({
            "a": 1,
            "b": 2
        });

        let comp = json!({
            "b": 2,
            "a": 1
        });

        assert_eq!(reference, comp);
    }

    fn test_import_export_loop() {
        let reference_string: String;
        match common::factorio_inflate(SAMPLE_BP) {
            Ok(_val) => {
                reference_string = _val
            },
            Err(e) => {
                panic!("{}", e);
            },
        }

        let reference_val: Value;


    }
}
