#![allow(unused)]
//! This crate contains the requisite blocks to draw a progress tree

pub const T_BRANCH: &str = "├";
pub const H_PUPE: &str = "──";
pub const V_PIPE: &str = "│";
pub const EMPTY_SPACE: &str = " ";
pub const EMPTY_BLOCK: &str = "  ";

/// Number of spaces separating the tree and it's name
pub const TREE_TO_STRING_SEP: u8 = 1;

/// Number of spaces separating one tree level from another
pub const TREE_TO_CHILD_SEP: u8 = 4;
