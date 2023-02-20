#![allow(unused)]
//! This module contains blocks that are needed to draw a progress tree.
//! Here's a look at how it should look like:
//!
//! src/
//! ├── lib.rs
//! ├── tracker.rs
//! ├── tree_blocks.rs
//! ├── types
//! │   ├── node_type
//! │   │   ├── tree_branch.rs
//! │   │   └── tree_node.rs
//! │   └── node_type.rs
//! └── types.rs
//!
//! Each child node in the tree starts off with a T_BRANCH,
//! except for the last one, where it starts with a L_BRANCH.
//! Then 2 dashes are used to make the branch a little longer.
//! After that, some space padding is used to make a gap between
//! the contents of a child node and the decorative characters.

pub const T_BRANCH: &str = "├";
pub const L_BRANCH: &str = "└";
pub const H_PIPE: &str = "──";
pub const V_PIPE: &str = "│";
pub const EMPTY_SPACE: &str = " ";
pub const EMPTY_BLOCK: &str = "  ";

/// Number of spaces separating the tree and it's name
pub const TREE_TO_STRING_SEP: usize = 1;

/// Number of spaces separating one tree level from another
pub const TREE_TO_CHILD_SEP: usize = 4;
