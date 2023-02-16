//! The tracker struct.

use std::sync::{Arc, RwLock};

use crate::types::NodeType;

use crossterm::style::Stylize;
use crossterm::{cursor, terminal, ExecutableCommand};

#[derive(Debug)]
#[allow(unused)]
pub struct Tracker {
    std_out: std::io::Stdout,

    /// Number of nodes and branches in the entire tree
    num_nodes: u16,

    /// Number of active nodes that are on display.
    num_active_nodes: u16,

    /// Starting point for the blueprint/book.
    root: Option<NodeType>,

    // statistics from the old version of progress
    read_books: u16,
    read_blueprints: u16,
    read_planners: u16,
    errors: u16,
}

impl Default for Tracker {
    fn default() -> Self {
        Self {
            std_out: std::io::stdout(),
            num_nodes: Default::default(),
            num_active_nodes: Default::default(),
            root: Default::default(),
            read_books: Default::default(),
            read_blueprints: Default::default(),
            read_planners: Default::default(),
            errors: Default::default(),
        }
    }
}

impl Tracker {
    pub fn new() -> Arc<RwLock<Self>> {
        let mut _tracker = Tracker::default();
        _tracker.std_out.execute(cursor::Hide).unwrap();

        Arc::new(RwLock::new(_tracker))
    }
}
