pub trait DirectChildrenGetter<T>: Fn(&T) -> Vec<T> {}
impl<T, F> DirectChildrenGetter<T> for F where F: Fn(&T) -> Vec<T> {}
