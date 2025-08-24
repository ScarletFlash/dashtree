use crate::tree_nodes::{RawTreeNode, TreeNode};

struct LastChild {
    last_child_index: usize,
    next_item_level: usize,
}

pub fn get_nodes_from_sorted_raw_nodes<T: Clone>(
    sorted_raw_nodes: Vec<RawTreeNode<T>>,
) -> Vec<TreeNode<T>> {
    let mut nodes_result = Vec::new();
    let mut last_children: Vec<LastChild> = Vec::new();

    for current_item_index in (0..sorted_raw_nodes.len()).rev() {
        let current_item = &sorted_raw_nodes[current_item_index];
        let current_item_level = current_item.level;

        let next_item_level = sorted_raw_nodes
            .get(current_item_index.clone().saturating_add(1))
            .map(|item| item.level)
            .unwrap_or(0);

        let is_last_child = next_item_level < current_item_level;
        let is_parent = current_item_level < next_item_level;

        if is_last_child {
            last_children.push(LastChild {
                last_child_index: current_item_index,
                next_item_level,
            });
        }

        let last_child_index = if is_parent {
            last_children
                .iter()
                .rev()
                .find(|lc| lc.next_item_level <= current_item_level)
                .map(|lc| lc.last_child_index)
                .expect(&format!(
                    "Last child is not defined for parent level {}",
                    current_item_level
                ))
        } else {
            current_item_index
        };

        nodes_result.insert(
            0,
            TreeNode::new(
                sorted_raw_nodes[current_item_index].payload.clone(),
                current_item_level,
                current_item.has_children,
                last_child_index,
            ),
        );
    }

    nodes_result
}
