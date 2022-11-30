pub struct BinaryTree<T> {
    v: Option<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self { v: None }
    }
}

impl<T> BinaryTree<T> {
    pub fn is_empty(&self) -> bool {
        self.v.is_none()
    }

    pub fn root(&self) -> Option<&T> {
        self.v.as_ref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty() {
        let t = BinaryTree::<i32>::new();
        assert!(t.is_empty());
        assert_eq!(t.root(), None);
    }
}
