mod tree_branch;
mod tree_node;

pub use tree_branch::TreeBranch;
pub use tree_node::TreeNode;

use crate::types::{NumLines, ProgressDisplayVariant, RwArc, TotalLines};

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

impl NodeType {
    /// Create a new branch variant
    pub fn new_branch<T>(branch_name: T) -> Self
    where
        T: ToString,
    {
        let mut _branch = TreeBranch::default();
        _branch.name = branch_name.to_string();

        NodeType::Branch(RwArc::new(_branch))
    }

    /// Create a new node variant
    pub fn new_node<T>(node_name: T) -> Self
    where
        T: ToString,
    {
        let mut _node = TreeNode::default();
        _node.name = node_name.to_string();

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
    pub fn update(&mut self, status: ProgressDisplayVariant, error_msg: Option<String>) {
        match self {
            NodeType::Branch(_b) => {
                let mut _b_write = _b.write().unwrap();
                _b_write.progress = status;
                if let Some(_err) = &error_msg {
                    _b_write.error_message = error_msg
                }
            }
            NodeType::Node(_n) => {
                _n.progress = status;
                if let Some(_err) = &error_msg {
                    _n.error_message = error_msg
                }
            }
        }
    }
}
