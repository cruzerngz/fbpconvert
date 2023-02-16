use crate::types::{ProgressDisplayVariant, RwArc, TotalLines, NumLines};

use crate::types::node_type::TreeBranch;

use super::NodeType;

/// Single node.
/// Contains information about:
/// - blueprint name
/// - progress of this node
#[derive(Clone, Debug, Default)]
pub struct TreeNode {
    /// Name of the node
    pub name: String,

    /// Progress of node
    pub progress: ProgressDisplayVariant,

    /// Error message (if any)
    pub error_message: Option<String>,

    /// Reference to parent branch (if any)
    pub parent: Option<RwArc<TreeBranch>>,
}

impl NumLines for TreeNode {
    fn num_lines(&self) -> usize {
        match &self.parent {
            Some(_parent) => {
                let mut lines = _parent.read().unwrap().num_lines();

                for (_name, _node) in &_parent.read().unwrap().children {
                    if let NodeType::Node(_n) = _node {
                        if self.name == _n.name {
                            lines += self.total_lines();
                            break;
                        }
                        else {
                            lines += _n.total_lines();
                        }
                    }
                }

                lines
            },
            None => self.total_lines(),
        }
    }
}

impl TotalLines for TreeNode {
    fn total_lines(&self) -> usize {
        if let Some(_) = self.error_message {
            1
        } else {
            0
        }
    }
}
