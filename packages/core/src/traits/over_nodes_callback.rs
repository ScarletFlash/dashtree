use crate::tree_nodes::TreeNode;

pub trait OverNodesCallback<T>: Fn(&TreeNode<T>, usize) {}
impl<T, F> OverNodesCallback<T> for F where F: Fn(&TreeNode<T>, usize) {}
