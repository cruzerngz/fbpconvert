//! This module handles writing progress to stdout

use std::io::Write;

use crossterm::{
    terminal,
    ExecutableCommand,
    QueueableCommand,
    cursor
};
use crossterm::style::Stylize;

/// Type of subcommand: import or export
pub enum CommandType {
    Import,
    Export
}

/// Type of blueprint: book or single blueprint
pub enum ProgressType {
    Book(String),
    Blueprint(String)
}

/// Progress tracker for data display.
pub struct Tracker {
    std_out: std::io::Stdout,
    pub command: CommandType,
    pub read_blueprints: u16,
    pub read_books: u16,
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
            errors: 0,
        }
    }

    /// Called when no error occurs
    pub fn ok(&mut self, progress_type: ProgressType) {
        let file_name: String;
        match progress_type {
            ProgressType::Book(_book) => {
                file_name = _book;
                self.read_books += 1;
            },
            ProgressType::Blueprint(_blueprint) => {
                file_name = _blueprint;
                self.read_blueprints += 1
            }
        }

        self.std_out.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        self.std_out.write(format!("{}\t{}\n", "ok".green().bold(), file_name).as_bytes()).unwrap();
        self.std_out.queue(cursor::MoveToPreviousLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Called when encountering an error
    pub fn error(&mut self, progress_type: ProgressType, err_msg: Option<String>) {
        let file_name: String;
        match progress_type {
            ProgressType::Book(_book) => {
                file_name = _book;
                self.read_books += 1;
            },
            ProgressType::Blueprint(_blueprint) => {
                file_name = _blueprint;
                self.read_blueprints += 1
            }
        }
        self.errors += 1;

        self.std_out.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        self.std_out.write(format!("{}\t{}\n", "err".red().bold(), file_name).as_bytes()).unwrap();
        match err_msg {
            Some(message) => {
                self.std_out.write(format!("{}\t{}\n", "msg".red().bold(), message).as_bytes()).unwrap();
            },
            None => ()
        }
        self.std_out.queue(cursor::MoveToNextLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Custom error, does not modify internal struct attributes
    pub fn error_additional(&mut self, err_msg: String) {
        self.std_out.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        self.std_out.write(format!("{}\t{}\n", "err".red().bold(), err_msg).as_bytes()).unwrap();
        self.std_out.queue(cursor::MoveToNextLine(1)).unwrap();

        self.std_out.flush().unwrap();
    }

    /// Updates stdout with final progress statistics
    pub fn complete(&mut self) {
        self.std_out.queue(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
        self.std_out.write(format!(
            "{}\t{}\n{}\t\t{}\n{}\t\t{}\n",
            "blueprints".green().bold(),
            self.read_blueprints,
            "books".green().bold(),
            self.read_books,
            "errors".green().bold(),
            self.errors
        ).as_bytes()).unwrap();

        self.std_out.queue(cursor::Show).unwrap();
        self.std_out.flush().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ProgressType::*;
    use std::{thread, time};

    #[test]
    fn progress_loop() {
        let mut progress_indicator = Tracker::new(CommandType::Import);

        for i in 1..50 {
            if i % 7 == 0 {
                progress_indicator.error(Book(format!("what? {}", i)), Some("Idk man some error occured".to_string()));
            } else {
                progress_indicator.ok(Blueprint(format!("everyting is going well: {}", i)));
            }
            thread::sleep(time::Duration::from_millis(15));
        }

        progress_indicator.complete();
    }

}
