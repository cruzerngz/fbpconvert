use std::fmt::Display;

use crate::types::{NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeComplete, TreeError};

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
                        } else {
                            lines += _n.total_lines();
                        }
                    }
                }

                lines
            }
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

impl TreeError for TreeNode {
    fn error<T: ToString>(&mut self, err_message: Option<T>) {
        self.progress = ProgressDisplayVariant::Error;
        self.error_message = {
            if let Some(_message) = err_message {
                Some(_message.to_string())
            } else {
                None
            }
        };

        if let Some(_parent) = &self.parent {
            _parent.write().unwrap().num_children_errors += 1;
            _parent.write().unwrap().update_internal_progress();
        }
    }
}

impl TreeComplete for TreeNode {
    fn complete(&mut self) {
        self.progress = ProgressDisplayVariant::Complete;
        if let Some(_parent) = &self.parent {
            _parent.write().unwrap().num_children_complete += 1;
            _parent.write().unwrap().update_internal_progress();
        }
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line = {
            if let Some(_err) = &self.error_message {
                format!("{}{}", self.name, _err)
            } else {
                self.name.clone()
            }
        };

        write!(f, "{line}")
    }
}

impl TreeNode {
    /// Create a new node.
    pub fn new<T: ToString>(name: T) -> TreeNode {
        let mut _node = Self::default();
        _node.name = name.to_string();

        _node
    }
}
