use crate::tree_nodes::RawTreeNode;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct TreeNode<T> {
    _base: RawTreeNode<T>,

    pub last_child_index: usize,
}

impl<T> Deref for TreeNode<T> {
    type Target = RawTreeNode<T>;

    fn deref(&self) -> &Self::Target {
        &self._base
    }
}

impl<T> DerefMut for TreeNode<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._base
    }
}

impl<T> TreeNode<T> {
    pub fn new(payload: T, level: usize, has_children: bool, last_child_index: usize) -> Self {
        Self {
            _base: RawTreeNode::new(payload, level, has_children),
            last_child_index,
        }
    }

    pub fn get_raw(self) -> RawTreeNode<T> {
        self._base
    }
}
