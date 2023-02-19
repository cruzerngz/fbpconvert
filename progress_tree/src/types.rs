//! Types used in this lib

mod node_type;

pub use node_type::{NodeType, TreeBranch, TreeNode};
// #![allow(unused)]

use std::{
    collections::HashSet,
    ops::Deref,
    sync::{Arc, RwLock},
};

/// Reference-counted read-write lock.
///
/// Wraps Arc and RwLock.
/// Behaves exactly like an ARC.
#[derive(Clone, Debug)]
pub struct RwArc<T> {
    data: Arc<RwLock<T>>,
}

impl<T> RwArc<T> {
    pub fn new(data: T) -> RwArc<T> {
        RwArc {
            data: Arc::new(RwLock::new(data)),
        }
    }
}

impl RwArc<TreeBranch> {
    /// Insert a node into a reference-counted branch.
    pub fn insert(&self, mut node: NodeType) {
        // add parent
        match &mut node {
            NodeType::Branch(_branch) => {
                _branch.write().unwrap().parent = Some(self.clone());
            }
            NodeType::Node(_node) => {
                _node.parent = Some(self.clone());
            }
        }

        let mut _pointer = self.write().unwrap();
        _pointer.children.insert(node.name(), node);
    }
}

impl<T> Deref for RwArc<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// The stage of progress in a node.
#[derive(Copy, Clone, Debug)]
pub enum ProgressDisplayVariant {
    Waiting,
    Running,
    Complete,
    Error,
}

impl Default for ProgressDisplayVariant {
    fn default() -> Self {
        Self::Waiting
    }
}

trait TotalLines {
    /// Return the number of lines occupied in the command line
    /// by a node and all of its children (if any).
    /// This is used to by crossterm to figure out how many lines to skip when adding/deleting lines.
    ///
    /// Branches occupy one line.
    ///
    /// Nodes (leaves) do not take up any space unless they contain an error message.
    fn total_lines(&self) -> usize;
}

trait NumLines {
    /// Number of lines (branches) from the top of ordered tree
    /// to the specified branch.    0-indexed.
    ///
    /// Used for indexing into and modifying a specific line in the terminal.
    /// See TotalLines for the total number of lines occupied by a branch
    fn num_lines(&self) -> usize;
}

trait TreeError {
    /// Mark a node with an error
    fn error<T: ToString>(&mut self, err_message: Option<T>);
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test the correctness of *Lines traits.
    #[cfg(test)]
    mod lines_traits {
        use super::*;

        /// Create a known tree for testing purposes
        /// The tree is created as follows (ordered):
        ///
        /// root
        /// - first branch
        ///     - first sub branch
        /// - first node (no error)
        /// - second branch
        ///     - second sub node (with error)
        ///
        /// This tree should occupy 4 (branches) + 1 (node with error) lines.
        fn create_known_tree() -> RwArc<TreeBranch> {
            let root = RwArc::new(TreeBranch::new("root"));

            let first_branch = RwArc::new(TreeBranch::new("first branch"));
            first_branch.insert(NodeType::new_branch("first sub branch"));
            root.insert(NodeType::Branch(first_branch));

            root.insert(NodeType::new_node("first node"));

            let second_branch = RwArc::new(TreeBranch::new("second branch"));
            let mut sub_node = TreeNode::default();
            sub_node.error(Some("some_message"));

            second_branch.insert(NodeType::Node(sub_node));
            root.insert(NodeType::Branch(second_branch));

            root
        }

        #[test]
        /// Test the TotalLines trait on stucts that implement it.
        fn total_lines_test() {
            let root = create_known_tree();

            assert_eq!(root.read().unwrap().total_lines(), 5);

            let first_node: TreeNode = {
                root.read()
                    .unwrap()
                    .children
                    .iter()
                    .find_map(|(_, _node)| match _node {
                        NodeType::Branch(_b) => None,
                        NodeType::Node(_n) => Some(_n.to_owned()),
                    })
                    .unwrap()
            };

            assert_eq!(
                first_node.num_lines(),
                0,
                "a node with no error (and has children with no errors) has the same line index as it's parent"
            );

            let last_branch = {
                root.read()
                    .unwrap()
                    .children
                    .iter()
                    .rev()
                    .find_map(|(_, _node)| match _node {
                        NodeType::Branch(_b) => Some(_b.clone()),
                        NodeType::Node(_n) => None,
                    })
                    .unwrap()
            };

            let first_sub_node_error: TreeNode = {
                last_branch
                    .read()
                    .unwrap()
                    .children
                    .iter()
                    .find_map(|(_, _node)| match _node {
                        NodeType::Branch(_) => None,
                        NodeType::Node(_n) => Some(_n.to_owned()),
                    })
                    .unwrap()
            };

            assert_eq!(last_branch.read().unwrap().num_lines(), 3);
            assert!(matches!(first_sub_node_error.error_message, Some(_)));
            assert_eq!(first_sub_node_error.num_lines(), 4);
        }

        #[test]
        fn num_lines_test() {
            let root = create_known_tree();

            assert_eq!(
                root.read().unwrap().num_lines(),
                0,
                "num lines to root should always be 0."
            );

            let first_branch = root
                .read()
                .unwrap()
                .children
                .iter()
                .find_map(|(_name, _node)| match _node {
                    NodeType::Branch(_b) => Some(_b.clone()),
                    NodeType::Node(_) => None,
                })
                .unwrap();

            assert_eq!(
                first_branch.read().unwrap().num_lines(),
                1,
                "num lines to first branch."
            );
        }
    }
}
