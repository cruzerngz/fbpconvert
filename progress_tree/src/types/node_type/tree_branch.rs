use std::{collections::BTreeMap, fmt::Display};

use crate::tree_blocks as blocks;
use crate::types::{
    NodeType, NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeComplete, TreeError,
};

/// A branch in the tree.
#[derive(Clone, Debug, Default)]
pub struct TreeBranch {
    pub name: String,
    /// Number of children immediately under the branch
    num_children: u16,

    pub num_children_errors: u16,

    pub num_children_complete: u16,

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
        // branch does not occupy any space when complete
        if let ProgressDisplayVariant::Complete = self.progress {
            return 0;
        }

        let mut lines: usize = 1; // the branch itself occupies 1 line

        for any_node in self.incomplete_iter() {
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

        // update errors in parent
        if let Some(_parent) = &self.parent {
            _parent.write().unwrap().num_children_errors += 1;
            _parent.write().unwrap().update_internal_progress();
        }
    }
}

impl TreeComplete for TreeBranch {
    fn complete(&mut self) {
        self.progress = ProgressDisplayVariant::Complete;
        if let Some(_parent) = &self.parent {
            _parent.write().unwrap().num_children_complete += 1;
            _parent.write().unwrap().update_internal_progress();
        }

        todo!()
    }
}

impl TreeBranch {
    /// Checks internal params and updates its progress.
    pub fn update_internal_progress(&mut self) {
        let curr_total = self.num_children_complete + self.num_children_errors;

        if curr_total < self.num_children {
            // not complete
            self.progress = ProgressDisplayVariant::Running;
        } else {
            // complete
            if self.num_children_errors > 0 {
                self.progress = ProgressDisplayVariant::CompleteWithError
            } else {
                self.progress = ProgressDisplayVariant::Complete
            }
        }
    }

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

    /// Returns an iterator over only the child branches.
    pub fn branch_iter(&self) -> impl Iterator<Item = &RwArc<TreeBranch>> {
        self.children.iter().filter_map(|(_, _node)| {
            if let NodeType::Branch(_branch) = &_node {
                Some(_branch)
            } else {
                None
            }
        })
    }

    /// Returns an iterator over incomplete child nodes
    pub fn incomplete_iter(&self) -> impl Iterator<Item = &NodeType> {
        let _iter = self.children.iter().filter_map(|(_, _node)| {
            if let ProgressDisplayVariant::Complete = _node.status() {
                None
            } else {
                Some(_node)
            }
        });

        _iter
    }

    /// Returns a mutable iterator over incomplete child nodes
    pub fn incomplete_iter_mut(&mut self) -> impl Iterator<Item = &mut NodeType> {
        let _iter = self.children.iter_mut().filter_map(|(_, _node)| {
            if let ProgressDisplayVariant::Complete = _node.status() {
                None
            } else {
                Some(_node)
            }
        });

        _iter
    }

    /// Internal function for Display.
    /// Show a tree at a specified indentation.
    /// Each level down adds 1 to the indent.
    /// The root takes in 0 as an indent.
    fn build_tree(&self) -> Vec<String> {
        let mut tree_lines: Vec<String> = Default::default();

        // add the tree root
        tree_lines.push(self.name.clone());

        // construct the top level tree first
        for _node in self.incomplete_iter() {
            match _node {
                // for branches, skip N lines, where N is the total size of the branch.
                NodeType::Branch(_branch) => {
                    for sub_line in 0.._branch.read().unwrap().total_lines() {
                        if sub_line == 0 {
                            tree_lines.push(format!(
                                "{}{}{}",
                                blocks::T_BRANCH,
                                blocks::H_PIPE,
                                blocks::EMPTY_SPACE.repeat(blocks::TREE_TO_STRING_SEP)
                            ))
                        } else {
                            tree_lines.push(format!(
                                "{}{}{}",
                                blocks::V_PIPE,
                                blocks::EMPTY_BLOCK,
                                blocks::EMPTY_SPACE.repeat(blocks::TREE_TO_STRING_SEP)
                            ))
                        }
                    }
                }
                // for nodes, append a new line if there are errors.
                NodeType::Node(tree_node) => {
                    if let (Some(_error), ProgressDisplayVariant::Error) =
                        (&tree_node.error_message, tree_node.progress)
                    {
                        tree_lines.push(format!(
                            "{}{}{}{}: {}",
                            blocks::T_BRANCH,
                            blocks::H_PIPE,
                            blocks::EMPTY_SPACE.repeat(blocks::TREE_TO_STRING_SEP),
                            tree_node.name,
                            _error
                        ))
                    }
                }
            }
        }

        if let Some((_name, _node)) = self.children.iter().last() {
            // change the T to L branch for the last child node
            let last_index = _node.num_lines() - self.num_lines();
            if let Some(_line) = tree_lines.get_mut(last_index) {
                if last_index > 0 {
                    _line.remove(0);
                    _line.insert_str(0, blocks::L_BRANCH);
                }
            }

            // remove dangling v_pipes if last child is a branch
            if let NodeType::Branch(_last_branch) = _node {
                // substitute total_lines - 1 number of v_pipes from end of vec // panics here
                let lines_to_remove = {
                    let _lines = _last_branch.read().unwrap().total_lines();
                    if _lines > 0 {
                        _lines - 1
                    } else {
                        0
                    }
                };

                for _line in tree_lines.iter_mut().rev().take(lines_to_remove) {
                    _line.remove(0);
                    _line.insert(0, ' ');
                }
            }
        }

        // add sub-trees after gaps are made
        for _b in self.branch_iter() {
            let _branch = _b.read().unwrap();

            // get the start and end indices relative to the parent tree.
            // note that the parent tree also has a relative index, so that needs to be subtracted.
            let start_pos = _branch.num_lines() - self.num_lines();

            let _branch_lines = _branch.build_tree();

            for (_parent_line, _child_line) in
                tree_lines[start_pos..].iter_mut().zip(_branch_lines.iter())
            {
                _parent_line.push_str(_child_line);
            }
        }

        tree_lines
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

impl Display for TreeBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = self.build_tree();
        let tree_as_str = lines.join("\n");

        write!(f, "{tree_as_str}")
    }
}

#[cfg(test)]
#[allow(unused)]
mod test {

    use super::TreeBranch;
    use crate::types::{
        NodeType, NumLines, ProgressDisplayVariant, RwArc, TotalLines, TreeError, TreeNode, TreeComplete,
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

    /// Create a more complicated tree.
    fn create_larger_known_tree() -> RwArc<TreeBranch> {
        let root = RwArc::new(TreeBranch::new("root"));

        let mut _branch_1_inner = TreeBranch::new("branch 1");
        let branch_1 = RwArc::new(_branch_1_inner.clone());
        let branch_1b = RwArc::new(_branch_1_inner.clone());
        let branch_2 = RwArc::new(TreeBranch::new("branch 2"));
        let branch_3 = RwArc::new(TreeBranch::new("branch 3"));

        branch_1b.insert(NodeType::new_branch("sub branch 1"));
        branch_1b.insert(NodeType::new_branch("sub_branch 2"));
        branch_1b.insert({
            let mut _node = TreeNode::new("sub node 1");
            _node.error(Some("random error"));

            NodeType::Node(_node)
        });
        branch_1b.insert({
            let mut _node = TreeNode::new("sub node 2");
            _node.error(Some("random error"));
            NodeType::Node(_node)
        });

        branch_1.insert(NodeType::new_branch("sub branch 1"));
        branch_1.insert(NodeType::new_branch("sub_branch 2"));
        branch_1.insert({
            let mut _node = TreeNode::new("sub node 1");
            _node.error(Some("random error"));

            NodeType::Node(_node)
        });
        branch_1.insert({
            let mut _node = TreeNode::new("sub node 2");
            _node.error(Some("random error"));
            NodeType::Node(_node)
        });
        branch_1.insert(NodeType::Branch(branch_1b));

        // branch_2.insert(NodeType::Branch(branch_1.clone()));
        // branch_2.insert(NodeType::Branch(branch_3.clone()));

        root.insert(NodeType::Branch(branch_1));
        root.insert(NodeType::Branch(branch_2));
        root.insert(NodeType::Branch(branch_3));
        root
        // branch_1
    }

    #[test]
    /// Test the error_iter() method
    fn error_iter_test() {
        let tree = create_known_tree();

        assert_eq!(tree.read().unwrap().num_children(), 10);
        assert_eq!(tree.read().unwrap().error_iter().count(), 5);
        assert_eq!(tree.read().unwrap().total_lines(), 6);
    }

    #[test]
    fn show_tree_test() {
        let tree = create_known_tree();
        let bigger_tree = create_larger_known_tree();

        println!(
            "Num lines for tree: {}",
            bigger_tree.read().unwrap().total_lines()
        );
        // println!("{}", tree.read().unwrap());
        println!("{}", bigger_tree.read().unwrap());

        for _branch in bigger_tree.read().unwrap().branch_iter() {
            println!("{}", _branch.read().unwrap());
        }
    }

    #[test]
    fn completion_tests() {
        let tree = create_known_tree();
        for item in tree.write().unwrap().incomplete_iter_mut() {
            // write() called on inner function while lock held on upper
            match item {
                NodeType::Branch(_b) => todo!(),
                NodeType::Node(_n) => _n.complete(),
            }
        }

        // tree.read().unwrap().children.ke

        if tree.is_poisoned() {
            panic!("rwlock not released");
        }

        let _tree = tree.read().unwrap();
        assert_eq!(_tree.num_children, _tree.num_children_complete);
        assert_eq!(_tree.num_children_errors, 0);
        assert!(matches!(_tree.progress, ProgressDisplayVariant::Complete));
    }
}
