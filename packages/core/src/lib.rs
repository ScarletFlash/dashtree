mod flat_tree;
mod flat_tree_error;
mod helpers;
mod traits;
mod tree_nodes;

mod wasm_bindings {
    use super::flat_tree::FlatTree as _FlatTree;
    use super::tree_nodes::TreeNode;
    use crate::flat_tree_error::FlatTreeError;
    use js_sys::{Array, Error, Function, Object, Reflect};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct FlatTree {
        _inner: _FlatTree<JsValue>,
    }

    #[wasm_bindgen]
    impl FlatTree {
        #[wasm_bindgen(constructor)]
        pub fn new(root_payloads: Vec<JsValue>, js_get_direct_children: &Function) -> Self {
            let get_direct_children_closure = move |parent: &JsValue| -> Vec<JsValue> {
                let js_direct_children = Array::from(
                    &js_get_direct_children
                        .call1(&JsValue::NULL, parent)
                        .unwrap(),
                );

                let mut direct_children = Vec::new();
                for child_index in 0..js_direct_children.length() {
                    direct_children.push(js_direct_children.get(child_index));
                }
                direct_children
            };

            Self {
                _inner: _FlatTree::new(root_payloads, get_direct_children_closure),
            }
        }

        pub fn get_nodes(&self) -> Vec<JsValue> {
            self._inner
                .get_nodes()
                .iter()
                .map(|node| self.node_to_js_value(node))
                .collect()
        }

        pub fn find_index(&self, predicate: &Function) -> Option<usize> {
            self._inner.find_index(|node| {
                let js_node = self.node_to_js_value(node);
                predicate
                    .call1(&JsValue::NULL, &js_node)
                    .unwrap()
                    .as_bool()
                    .unwrap()
            })
        }

        pub fn get_by_index(&self, target_index: usize) -> Result<JsValue, Error> {
            self._inner
                .get(target_index)
                .map(|node| self.node_to_js_value(node))
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        pub fn for_each(&self, callback: &Function) {
            self._inner.for_each(|node, index| {
                let js_node = self.node_to_js_value(node);
                let js_index = JsValue::from(index as u32);
                callback.call2(&JsValue::NULL, &js_node, &js_index).unwrap();
            });
        }

        pub fn for_each_child(
            &self,
            parent_index: usize,
            callback: &Function,
        ) -> Result<(), Error> {
            self._inner
                .for_each_child(parent_index, |node, index| {
                    let js_node = self.node_to_js_value(node);
                    let js_index = JsValue::from(index as u32);
                    callback.call2(&JsValue::NULL, &js_node, &js_index).unwrap();
                })
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        pub fn for_each_parent(
            &self,
            target_index: usize,
            callback: &Function,
        ) -> Result<(), Error> {
            self._inner
                .for_each_parent(target_index, |node, index| {
                    let js_node = self.node_to_js_value(node);
                    let js_index = JsValue::from(index as u32);
                    callback.call2(&JsValue::NULL, &js_node, &js_index).unwrap();
                })
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        pub fn get_sub_tree_by_index(&self, root_index: usize) -> Result<Vec<JsValue>, Error> {
            self._inner
                .get_sub_tree_by_parent_index(root_index, None)
                .map(|nodes| {
                    nodes
                        .into_iter()
                        .map(|node| self.node_to_js_value(&node))
                        .collect()
                })
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        pub fn replace_payload(
            &mut self,
            target_index: usize,
            payload: JsValue,
        ) -> Result<(), Error> {
            self._inner
                .replace_payload(target_index, payload)
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        pub fn delete(&mut self, target_index: usize) -> Result<(), Error> {
            self._inner
                .delete(target_index)
                .map_err(|internal_error| self.error_to_js_value(internal_error))
        }

        fn node_to_js_value(&self, node: &TreeNode<JsValue>) -> JsValue {
            let object_result = Object::new();

            Reflect::set(&object_result, &JsValue::from("payload"), &node.payload).unwrap();

            Reflect::set(
                &object_result,
                &JsValue::from("level"),
                &JsValue::from(node.level as u32),
            )
            .unwrap();

            Reflect::set(
                &object_result,
                &JsValue::from("hasChildren"),
                &JsValue::from(node.has_children),
            )
            .unwrap();

            Reflect::set(
                &object_result,
                &JsValue::from("lastChildIndex"),
                &JsValue::from(node.last_child_index as u32),
            )
            .unwrap();

            object_result.into()
        }

        fn error_to_js_value(&self, error: FlatTreeError) -> Error {
            match error {
                FlatTreeError::NodeWithSpecifiedIndexIsMissing { index } => Error::new(
                    format!("Node with index=[{index}] is requested, but missing").as_str(),
                ),
            }
        }
    }
}

pub use wasm_bindings::FlatTree;
