#![allow(unused)]

use std::{
    collections::HashMap,
    default,
    ops::{Index, IndexMut, Deref, DerefMut},
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex,
    },
};

/// Type alias for reference-counted mutable objects.
type SmartPointer<T> = Arc<Mutex<T>>;

/// Progress tracking struct.
pub struct Progress {
    head: Option<SmartPointer<ProgressNode>>,
    map: HashMap<String, SmartPointer<ProgressNode>>,
}

/// Represents a single node in a blueprint tree.
/// This can be the top-level blueprint book, or the deepest
/// nested blueprint in a blueprint book.
///
/// Blueprint books contain their own blueprints and/or books.
#[derive(Debug)]
pub enum ProgressNode {
    Blueprint(Blueprint),
    Book(Book),
}

/// Represents the state of a task (ser/deser a node)
#[derive(Debug)]
pub enum ProgressInfo {
    Ongoing,
    Complete,
    /// Contains the error message
    Error(Option<String>),
    /// Error with a child node
    ChildError,
}

#[derive(Debug)]
pub struct Blueprint {
    name: String,
    progress: ProgressInfo,
}

#[derive(Debug)]
pub struct Book {
    name: String,
    progress: ProgressInfo,
    blueprints: Vec<SmartPointer<ProgressNode>>,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            head: Default::default(),
            map: Default::default(),
        }
    }
}

impl Deref for Progress {
    type Target = HashMap<String, SmartPointer<ProgressNode>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Progress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

// impl Index<String> for Progress {
//     type Output = Option<SmartPointer<ProgressNode>>;

//     fn index(&self, index: String) -> &Self::Output {
//         let res = self.map.get(&index);
//         // return &res;
//         // todo!()

//         todo!()
//     }
// }

// impl IndexMut<String> for Progress {
//     fn index_mut(&mut self, index: String) -> &mut Self::Output {
//         todo!()
//     }
// }

/// Message packet
// pub struct ProgressMessage {}

// impl Progress {
//     pub fn new() -> Progress {
//         todo!()
//     }

//     /// Start and await any progress messages
//     pub fn start(&mut self) -> Sender<ProgressMessage> {
//         todo!()
//     }

//     pub fn stop(&mut self) {}

//     fn start_task(prog: Progress) {}
// }

#[cfg(test)]
mod tests {
    use super::*;
}
