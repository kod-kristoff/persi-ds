
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
