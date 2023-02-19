use std::collections::BTreeMap;

use crate::types::{NodeType, NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeError};

/// A branch in the tree.
#[derive(Clone, Debug, Default)]
pub struct TreeBranch {
    pub name: String,
    /// Number of children immediately under the branch
    num_children: u16,

    /// Number of children under **all** its constituent branches
    num_children_all: u16,

    /// Progress of the branch
    pub progress: ProgressDisplayVariant,

    /// Error message (if any)
    pub error_message: Option<String>,

    /// Ordered map of children
    pub children: BTreeMap<String, NodeType>,

    /// Reference to parent branch, if any.
    pub parent: Option<RwArc<TreeBranch>>,
}

impl TotalLines for TreeBranch {
    fn total_lines(&self) -> usize {
        let mut lines: usize = 1; // the branch itself occupies 1 line

        for (_, any_node) in self.children.iter() {
            match any_node {
                NodeType::Branch(_branch) => lines += _branch.read().unwrap().total_lines(),
                NodeType::Node(_node) => lines += _node.total_lines(),
            }
        }

        lines
    }
}

impl NumLines for TreeBranch {
    fn num_lines(&self) -> usize {
        let mut line_count: usize = 0;

        match &self.parent {
            Some(_parent) => {
                let _parent_pointer = _parent.read().unwrap();

                // add num_lines up to current parent
                line_count += _parent_pointer.num_lines();

                for (name, node) in _parent_pointer.children.iter() {
                    if name.eq(&self.name) {
                        line_count += 1;
                        break; // get the count of lines right before current branch
                    } else {
                        line_count += node.total_lines();
                    }
                }
            }
            None => (),
        }

        line_count
    }
}

impl TreeError for TreeBranch {
    fn error<T: ToString>(&mut self, err_message: Option<T>) {
        self.progress = ProgressDisplayVariant::Error;
        self.error_message = {
            if let Some(_message) = err_message {
                Some(_message.to_string())
            } else {
                None
            }
        };
    }
}

impl TreeBranch {
    /// Create a new instance of TreeBranch
    pub fn new<T>(name: T) -> Self
    where
        T: ToString,
    {
        let mut _branch = TreeBranch::default();
        _branch.name = name.to_string();

        _branch
    }

    /// Insert a new node to the branch.
    ///
    /// Insertion order is preserved.
    pub fn insert(&mut self, self_ref: RwArc<Self>, mut item: NodeType) {
        // add a reference to the parent branch
        // if we are inserting a branch type
        if let NodeType::Branch(_branch) = &mut item {
            _branch.write().unwrap().parent = Some(self_ref);
        }

        self.children.insert(item.name(), item);
        self.num_children += 1;
        self.num_children_all += 1;
    }

    /// Returns the number of children immediately under itself.
    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    /// Returns the percentage of immediate children that have completed conversion.
    pub fn percentage_completion(&self) -> f32 {
        let total = self.num_children();
        let error = self.error_iter().count();

        1_f32 - (error / total) as f32
    }

    /// Returns an iterator over any child nodes that contain an error.
    pub fn error_iter(&self) -> impl Iterator<Item = (&String, &NodeType)> {
        self.children.iter().filter(|(_, _node)| match _node {
            NodeType::Branch(_b) => {
                matches!(_b.read().unwrap().progress, ProgressDisplayVariant::Error)
            }
            NodeType::Node(_n) => matches!(_n.progress, ProgressDisplayVariant::Error),
        })
    }

    #[cfg(notset)]
    /// Shows the tree, at a specified indentation
    fn show_tree(&self, indent: usize) {
        println!("{}{}", "  ".repeat(indent), self.name);
        for (_name, _node) in self.children.iter() {
            match _node {
                NodeType::Branch(_branch) => _branch.read().unwrap().show_tree(indent + 1),
                NodeType::Node(_) => println!("{}{}", "  ".repeat(indent + 1), _name),
            }
        }
    }

    #[cfg(notset)]
    /// Propogate any changes upwards recursively to the number of children.
    fn back_propogate(&mut self, change: i16) {
        if let Some(_parent) = &mut self.parent {
            let mut _parent_pointer = _parent.write().unwrap();
            _parent_pointer.num_children = (_parent_pointer.num_children as i16 + change) as u16;
            _parent_pointer.num_children_all =
                (_parent_pointer.num_children_all as i16 + change) as u16;

            _parent_pointer.back_propogate(change);
        }
    }
}

#[cfg(test)]
#[allow(unused)]
mod test {

    use super::TreeBranch;
    use crate::types::{
        NodeType, NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeError, TreeNode,
    };

    /// Creates a tree containing 10 children.
    /// Children inserted in even-numbered intervals are marked with an error.
    /// The other children are initialised to the default progress state.
    fn create_known_tree() -> RwArc<TreeBranch> {
        let root = RwArc::new(TreeBranch::new("root"));

        let children: Vec<TreeNode> = {
            let _iter = (0..10).into_iter().map(|num| {
                let mut _node = TreeNode::new(format!("node number {num}"));

                if num % 2 == 0 {
                    _node.error(Some(format!("node {num} error")))
                }

                _node
            });

            _iter.collect()
        };

        for child in children.into_iter() {
            root.insert(NodeType::Node(child))
        }

        root
    }

    #[test]
    /// Test the error_iter() method
    fn error_iter_test() {
        let tree = create_known_tree();

        assert_eq!(tree.read().unwrap().num_children(), 10);
        assert_eq!(tree.read().unwrap().error_iter().count(), 5);
        assert_eq!(tree.read().unwrap().total_lines(), 6);
    }
}
