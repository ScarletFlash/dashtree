pub trait UsizeUnaryOperations {
    fn increment(self) -> Self;
    fn decrement(self) -> Self;
    fn increment_mut(&mut self);
    fn decrement_mut(&mut self);
}

impl UsizeUnaryOperations for usize {
    fn increment(self) -> Self {
        self.saturating_add(1)
    }

    fn decrement(self) -> Self {
        self.saturating_sub(1)
    }

    fn increment_mut(&mut self) {
        *self = self.saturating_add(1);
    }

    fn decrement_mut(&mut self) {
        *self = self.saturating_sub(1);
    }
}
