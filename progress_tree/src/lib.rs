//! This library crate provides an easy to comprehend visual
//! on the unpacking/repacking of blueprints. It attempts to
//! visually imitate the unix `tree` command.
//!

pub mod tracker;
pub mod types;

mod tree_blocks;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use std::io::stdout;
    use std::io::{Read, Write};
    use std::time::Duration;

    use crossterm::style::Stylize;
    use crossterm::{cursor, execute, terminal, ExecutableCommand, QueueableCommand};

    /// # Prototype documentation
    /// Let's say that the progress tracker is a type.
    /// It can be initialized with ProgressTracker::new()
    /// Once that has been initialized,
    /// The instance is passed down into each recursive blueprint import/export call as an ARC reference
    /// Each function call updates the tracker with the individual progress of a blueprint.
    /// Only books (branches) will be displayed in the terminal progress.
    /// It's children will be shown on a status bar.
    ///
    /// Each blueprint/book write function updates the tracker by a set of public functions.
    /// The planner can be indexed using the book/blueprint name.
    ///
    /// Requirements for the blueprint book data structure:
    /// - asdad
    ///
    #[allow(dead_code)]
    fn this_is_the_prototype_documentation() {
        let size: Vec<u32> = vec![0, 1, 2, 3];
    }

    // #[test]
    fn trying_out_crossterm_features() {
        let mut std_out = stdout();

        std_out.write("This is line 1\n".as_bytes()).unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        std_out.flush().unwrap();

        std_out.write("This is line 2\n".as_bytes()).unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        std_out.flush().unwrap();

        std_out.write("This is line 3\n".as_bytes()).unwrap();
        std::thread::sleep(Duration::from_millis(1000));
        std_out.flush().unwrap();

        std_out.queue(cursor::SavePosition).unwrap();

        std_out.queue(cursor::MoveToPreviousLine(2)).unwrap();
        std_out
            .queue(terminal::Clear(terminal::ClearType::CurrentLine))
            .unwrap();

        std_out.flush().unwrap();
        std::thread::sleep(Duration::from_millis(1000));

        std_out.queue(cursor::RestorePosition).unwrap();
        std_out.flush().unwrap();
        std::thread::sleep(Duration::from_millis(1000));
    }
}
