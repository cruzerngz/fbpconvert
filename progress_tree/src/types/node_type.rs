mod tree_branch;
mod tree_node;

use std::fmt::Display;

pub use tree_branch::TreeBranch;
pub use tree_node::TreeNode;

use crate::types::{NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeError};

use super::TreeComplete;

/// Type of node in the blueprint tree.
#[derive(Clone, Debug)]
pub enum NodeType {
    /// Equivalent to blueprint books.
    ///
    /// Only branches are reference-counted.
    Branch(RwArc<TreeBranch>),

    /// Equivalent to everything other than blueprint books.
    ///
    /// This variant is owned by a branch.
    Node(TreeNode),
}

#[derive(Clone, Debug, Default)]
/// A status indicator that sits between the tree branches and
/// its contents.
struct ProgressIndicator {
    complete: u16,
    total: u16,
}

impl Display for ProgressIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:5}/{:5}]", self.complete, self.total)
    }
}

#[allow(dead_code)]
impl ProgressIndicator {
    pub fn new(total: u16) -> Self {
        let mut _indicator = Self::default();
        _indicator.total = total;

        _indicator
    }

    /// Adds 1 to the completed count
    pub fn add(&mut self) {
        if self.complete != self.total {
            self.complete += 1
        }
    }

    pub fn update(&mut self, count: u16) {
        self.complete = count;
    }
}

impl TotalLines for NodeType {
    fn total_lines(&self) -> usize {
        match self {
            NodeType::Branch(_branch) => _branch.read().unwrap().total_lines(),
            NodeType::Node(_node) => _node.total_lines(),
        }
    }
}

impl NumLines for NodeType {
    fn num_lines(&self) -> usize {
        match self {
            NodeType::Branch(_b) => _b.read().unwrap().num_lines(),
            NodeType::Node(_n) => _n.num_lines(),
        }
    }
}

impl TreeError for NodeType {
    fn error<T: ToString>(&mut self, err_message: Option<T>) {
        match self {
            NodeType::Branch(_b) => _b.write().unwrap().error(err_message),
            NodeType::Node(_n) => _n.error(err_message),
        }
    }
}

impl TreeComplete for NodeType {
    fn complete(&mut self) {
        match self {
            NodeType::Branch(_b) => _b.write().unwrap().complete(),
            NodeType::Node(_n) => _n.complete(),
        }
    }
}

impl NodeType {
    /// Create a new branch variant
    pub fn new_branch<T>(branch_name: T) -> Self
    where
        T: ToString,
    {
        let mut _branch = TreeBranch::new(branch_name);

        NodeType::Branch(RwArc::new(_branch))
    }

    /// Create a new node variant
    pub fn new_node<T>(node_name: T) -> Self
    where
        T: ToString,
    {
        let mut _node = TreeNode::new(node_name);

        NodeType::Node(_node)
    }

    /// Returns the name of the node
    pub fn name(&self) -> String {
        match self {
            NodeType::Branch(_branch) => _branch.read().unwrap().name.clone(),
            NodeType::Node(_node) => _node.name.clone(),
        }
    }

    pub fn status(&self) -> ProgressDisplayVariant {
        match self {
            NodeType::Branch(_b) => _b.read().unwrap().progress,
            NodeType::Node(_n) => _n.progress,
        }
    }

    /// Update the node with the program's status.
    pub fn update(&mut self, status: ProgressDisplayVariant) {
        match self {
            NodeType::Branch(_b) => {
                let mut _b_write = _b.write().unwrap();
                _b_write.progress = status;
            }
            NodeType::Node(_n) => {
                _n.progress = status;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::ProgressIndicator;

    #[test]
    fn progress_indicator_test() {
        let mut indicator = ProgressIndicator::new(100);

        println!("{}", &indicator);
        indicator.update(5);
        println!("{}", &indicator);
    }
}
