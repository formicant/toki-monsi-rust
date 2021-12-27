pub trait VecExt<T> {
    fn clone_and_prepend(&self, item: T) -> Self;
    fn clone_and_append(&self, item: T) -> Self;
}

impl<T> VecExt<T> for Vec<T> where T: Clone {
    fn clone_and_prepend(&self, item: T) -> Self {
        let mut prepended = Vec::with_capacity(self.len() + 1);
        prepended.push(item);
        prepended.extend_from_slice(&self);
        prepended
    }
    
    fn clone_and_append(&self, item: T) -> Self {
        let mut appended = Vec::with_capacity(self.len() + 1);
        appended.extend_from_slice(&self);
        appended.push(item);
        appended
    }
}
