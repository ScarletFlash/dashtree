use crate::traits::{DirectChildrenGetter, UsizeUnaryOperations};
use crate::tree_nodes::RawTreeNode;
use std::collections::VecDeque;

struct PayloadWithLevel<T> {
    payload: T,
    level: usize,
}

impl<T> PayloadWithLevel<T> {
    pub fn new(payload: T) -> Self {
        Self { payload, level: 0 }
    }

    pub fn with_level(mut self, level: usize) -> Self {
        self.level = level;
        self
    }
}

pub fn get_sorted_raw_nodes_from_payloads<T, F>(
    root_payloads: Vec<T>,
    get_direct_children: F,
) -> Vec<RawTreeNode<T>>
where
    F: DirectChildrenGetter<T>,
{
    let mut raw_nodes_result = Vec::new();

    let mut items_to_unwrap: VecDeque<PayloadWithLevel<T>> = root_payloads
        .into_iter()
        .map(|payload| PayloadWithLevel::new(payload))
        .collect();

    while let Some(current_item) = items_to_unwrap.pop_front() {
        let current_item_direct_children = get_direct_children(&current_item.payload);
        let has_children = !current_item_direct_children.is_empty();

        raw_nodes_result.push(RawTreeNode::new(
            current_item.payload,
            current_item.level,
            has_children,
        ));

        if !has_children {
            continue;
        }

        for payload in current_item_direct_children.into_iter().rev() {
            let item = PayloadWithLevel::new(payload).with_level(current_item.level.increment());
            items_to_unwrap.push_front(item);
        }
    }

    raw_nodes_result
}
