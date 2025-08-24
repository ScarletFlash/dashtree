#[derive(Debug, Clone)]
pub struct RawTreeNode<T> {
    pub payload: T,
    pub level: usize,
    pub has_children: bool,
}

impl<T> RawTreeNode<T> {
    pub fn new(payload: T, level: usize, has_children: bool) -> Self {
        Self {
            payload,
            level,
            has_children,
        }
    }
}
