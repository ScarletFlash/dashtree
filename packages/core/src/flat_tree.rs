use crate::flat_tree_error::FlatTreeError;
use crate::helpers::{get_nodes_from_sorted_raw_nodes, get_sorted_raw_nodes_from_payloads};
use crate::traits::{DirectChildrenGetter, OverNodesCallback, UsizeUnaryOperations};
use crate::tree_nodes::{RawTreeNode, TreeNode};

pub struct FlatTree<T> {
    nodes: Vec<TreeNode<T>>,
}

impl<T: Clone> FlatTree<T> {
    pub fn new<F>(root_payloads: Vec<T>, get_direct_children: F) -> Self
    where
        F: DirectChildrenGetter<T>,
    {
        let raw_nodes = get_sorted_raw_nodes_from_payloads(root_payloads, get_direct_children);

        Self {
            nodes: get_nodes_from_sorted_raw_nodes(raw_nodes),
        }
    }

    pub fn get_nodes(&self) -> &Vec<TreeNode<T>> {
        &self.nodes
    }

    pub fn find_index<F>(&self, predicate: F) -> Option<usize>
    where
        F: Fn(&TreeNode<T>) -> bool,
    {
        self.nodes.iter().position(predicate)
    }

    pub fn get(&self, index: usize) -> Result<&TreeNode<T>, FlatTreeError> {
        self.nodes
            .get(index)
            .ok_or_else(|| FlatTreeError::NodeWithSpecifiedIndexIsMissing { index })
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut TreeNode<T>, FlatTreeError> {
        self.nodes
            .get_mut(index)
            .ok_or_else(|| FlatTreeError::NodeWithSpecifiedIndexIsMissing { index })
    }

    pub fn for_each<F>(&self, callback: F)
    where
        F: OverNodesCallback<T>,
    {
        for (index, node) in self.nodes.iter().enumerate() {
            callback(node, index);
        }
    }

    pub fn for_each_child<F>(&self, parent_index: usize, callback: F) -> Result<(), FlatTreeError>
    where
        F: OverNodesCallback<T>,
    {
        let parent = self.get(parent_index)?;
        let last_child_index = parent.last_child_index;

        for current_index in parent_index..=last_child_index {
            let current_node = self.get(current_index)?;
            callback(current_node, current_index);
        }

        Ok(())
    }

    pub fn for_each_parent<F>(&self, child_index: usize, callback: F) -> Result<(), FlatTreeError>
    where
        F: OverNodesCallback<T>,
    {
        let child = self.get(child_index)?;

        let mut current_level = child.level.decrement();
        for current_index in (0..child_index).rev() {
            let current_node = self.get(current_index)?;
            if current_node.level != current_level {
                continue;
            }

            callback(current_node, current_index);
            current_level.decrement_mut();
        }

        Ok(())
    }

    pub fn get_sub_tree_by_parent_index(
        &self,
        root_index: usize,
        root_level_overwrite: Option<usize>,
    ) -> Result<Vec<TreeNode<T>>, FlatTreeError> {
        let root = self.get(root_index)?;

        let mut sub_tree_result = self.nodes[root_index..=root.last_child_index].to_vec();

        if let Some(root_level) = root_level_overwrite {
            if root.level != root_level {
                let level_delta = root_level - root.level;

                sub_tree_result.iter_mut().for_each(|node| {
                    node.level += level_delta;
                });
            }
        }

        Ok(sub_tree_result)
    }

    pub fn replace_payload(&mut self, index: usize, payload: T) -> Result<(), FlatTreeError> {
        let target = self.get_mut(index)?;
        target.payload = payload;
        Ok(())
    }

    pub fn delete(&mut self, root_index: usize) -> Result<(), FlatTreeError> {
        let root = self.get(root_index)?;

        let modified_raw_nodes: Vec<RawTreeNode<T>> = self
            .nodes
            .clone()
            .drain(root_index..root.last_child_index)
            .into_iter()
            .map(|node| node.get_raw())
            .collect();

        self.nodes = get_nodes_from_sorted_raw_nodes(modified_raw_nodes);

        Ok(())
    }
}
