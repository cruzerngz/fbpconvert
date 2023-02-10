//! Types used in this lib



#[derive(Clone, Debug)]
pub struct TreeNode {

}

#[derive(Clone, Debug)]
pub struct TreeBranch {

}


/// Type of node in the blueprint tree.
/// Branches - akin to blueprint books
/// Nodes - everything else (blueprints, planners)
#[derive(Clone, Debug)]
pub enum NodeType {
    Branch(TreeBranch),
    Node(TreeNode)
}
