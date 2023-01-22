//! This module handles writing progress to stdout

use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use crossterm::style::Stylize;
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};

/// Type of blueprint: take from common module
pub use crate::common::BlueprintType as ProgressType;

/// Type of subcommand: import or export
pub enum CommandType {
    Import,
    Export,
}

/// Progress tracker for data display.
pub struct Tracker {
    std_out: std::io::Stdout,
    pub command: CommandType,
    pub read_blueprints: u16,
    pub read_books: u16,
    pub read_planners: u16,
    pub errors: u16,
}

impl Tracker {
    pub fn new(command: CommandType) -> Tracker {
        let mut _stdout = std::io::stdout();
        _stdout.execute(cursor::Hide).unwrap();

        Tracker {
            std_out: _stdout,
            command,
            read_blueprints: 0,
            read_books: 0,
            read_planners: 0,
            errors: 0,
        }
    }

    /// Tracker enclosed in reference-counted mutex
    pub fn new_sync(command: CommandType) -> Arc<Mutex<Tracker>> {
        let mut _stdout = std::io::stdout();
        _stdout.execute(cursor::Hide).unwrap();

        let _tracker = Tracker {
            std_out: _stdout,
            command,
            read_blueprints: 0,
            read_books: 0,
            read_planners: 0,
            errors: 0,
        };

        Arc::new(Mutex::new(_tracker))
    }

    /// Called when no error occurs
    pub fn ok(&mut self, progress_type: ProgressType) {
        let file_name: String;
        match progress_type {
            ProgressType::Book(_book) => {
                file_name = _book;
                self.read_books += 1;
            }
            ProgressType::Blueprint(_blueprint) => {
                file_name = _blueprint;
                self.read_blueprints += 1
            }
            ProgressType::UpgradePlanner(_planner) => {
                file_name = _planner;
                self.read_planners += 1;
            },
            ProgressType::DeconPlanner(_planner) => {
                file_name = _planner;
                self.read_planners += 1;
            },
            ProgressType::Invalid => return
            // {
            //     file_name = "invalid?".to_string()
            // },
        }

        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(format!("{}\t{}\n", "ok".green().bold(), file_name).as_bytes())
            .unwrap();
        self.std_out.queue(cursor::MoveToPreviousLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Custom non-error message, may be overwritten
    pub fn msg_temp(&mut self, ok_msg: String) {
        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(format!("{}\t{}\n", "msg".green().bold(), ok_msg).as_bytes())
            .unwrap();
        self.std_out.queue(cursor::MoveToPreviousLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Custom non-error message, does not modify internal struct attributes
    /// Message not overwritten
    pub fn msg(&mut self, ok_msg: String) {
        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(format!("{}\t{}\n", "msg".green().bold(), ok_msg).as_bytes())
            .unwrap();
        self.std_out.queue(cursor::MoveToNextLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Called when encountering an error
    pub fn error(&mut self, progress_type: ProgressType, err_msg: Option<String>) {
        let file_name: String;
        match progress_type {
            ProgressType::Book(_book) => {
                file_name = _book;
                self.read_books += 1;
            }
            ProgressType::Blueprint(_blueprint) => {
                file_name = _blueprint;
                self.read_blueprints += 1
            }
            ProgressType::Invalid => file_name = "invalid contents".to_string(),
            ProgressType::UpgradePlanner(_planner) => file_name = _planner,
            ProgressType::DeconPlanner(_planner) => file_name = _planner,
        }
        self.errors += 1;

        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(format!("{}\t{}\n", "err".red().bold(), file_name).as_bytes())
            .unwrap();
        match err_msg {
            Some(message) => {
                self.std_out
                    .write(format!("{}\t{}\n", "msg".red().bold(), message).as_bytes())
                    .unwrap();
            }
            None => (),
        }
        self.std_out.queue(cursor::MoveToNextLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Custom error, does not modify internal struct attributes
    pub fn error_additional(&mut self, err_msg: String) {
        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(format!("{}\t{}\n", "err".red().bold(), err_msg).as_bytes())
            .unwrap();
        self.std_out.queue(cursor::MoveToNextLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Updates stdout with final progress statistics
    pub fn complete(&mut self) {
        self.std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();
        self.std_out
            .write(
                format!(
                    "{}\t\t{}\n{}\t{}\n{}\t{}\n{}\t\t{}\n",
                    "books".green().bold(),
                    self.read_books,
                    "blueprints".green().bold(),
                    self.read_blueprints,
                    "planners".green().bold(),
                    self.read_planners,
                    "errors".green().bold(),
                    self.errors
                )
                .as_bytes(),
            )
            .unwrap();

        self.std_out.queue(cursor::Show).unwrap();
        self.std_out.flush().unwrap();
    }

    /// Waits for a keypress before continuing
    pub fn pause(message: String) {
        std::io::stdout()
            .write(
                format!(
                    "{}\n{}",
                    message,
                    "press any key to continue...".green().bold()
                )
                .as_bytes(),
            )
            .unwrap();
        std::io::stdout().queue(cursor::MoveToNextLine(1)).unwrap();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read(&mut [0]).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{thread, time};
    use ProgressType::*;

    #[test]
    fn progress_loop() {
        let mut progress_indicator = Tracker::new(CommandType::Import);

        for i in 1..50 {
            if i % 7 == 0 {
                progress_indicator.error(
                    Book(format!("what? {}", i)),
                    Some("Idk man some error occured".to_string()),
                );
            } else {
                progress_indicator.ok(Blueprint(format!("everyting is going well: {}", i)));
            }
            thread::sleep(time::Duration::from_millis(15));
        }

        progress_indicator.complete();
    }
}
