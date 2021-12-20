use crate::unsync::list::List;
use std::rc::Rc;

#[derive(Debug)]
pub struct Tree<T> {
    root: Option<Rc<TreeNode<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn leaf(x: T) -> Self {
        Tree {
            root: Some(Rc::new(TreeNode::leaf(x))),
        }
    }

    pub fn tree(x: T, children: List<Self>) -> Self {
        Tree {
            root: Some(Rc::new(TreeNode::new(x, children))),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn root(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.element)
    }

    pub fn children(&self) -> Option<&List<Tree<T>>> {
        self.root.as_ref().map(|node| &node.children)
    }
}

impl<T> Clone for Tree<T> {
    fn clone(&self) -> Self {
        Tree {
            root: self.root.clone(),
        }
    }
}

impl<T> PartialEq for Tree<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match self.root.as_ref() {
            None => other.is_empty(),
            Some(x) => match other.root.as_ref() {
                None => false,
                Some(y) => x == y,
            },
        }
    }
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    children: List<Tree<T>>,
}

impl<T> TreeNode<T> {
    fn leaf(element: T) -> Self {
        Self {
            element,
            children: List::new(),
        }
    }

    fn new(element: T, children: List<Tree<T>>) -> Self {
        Self { element, children }
    }
}

impl<T> PartialEq for TreeNode<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element && self.children == other.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unsynced_list;

    #[test]
    fn new_creates_empty_tree() {
        let tree = Tree::<&str>::new();

        assert!(tree.is_empty());
        assert_eq!(tree.root(), None);
        assert_eq!(tree.children(), None);
    }

    #[test]
    fn leaf_creates_tree_w_no_children() {
        let tree = Tree::leaf(5);

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&5));
        assert!(tree.children().unwrap().is_empty());
    }

    #[test]
    fn tree_creates_tree_w_no_children() {
        let tree = Tree::tree("a", List::new());

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&"a"));
        assert!(tree.children().unwrap().is_empty());
    }

    #[test]
    fn tree_creates_tree_w_children() {
        let children = unsynced_list!(Tree::leaf("b"), Tree::leaf("c"));
        let tree = Tree::tree("a", children.clone());

        assert!(!tree.is_empty());
        assert_eq!(tree.root(), Some(&"a"));
        assert!(!tree.children().unwrap().is_empty());
        assert_eq!(tree.children(), Some(&children));
    }

    #[test]
    fn test_partial_eq() {
        let t1 = Tree::<i32>::new();
        let t2 = Tree::<i32>::new();
        let t3 = Tree::leaf(4);
        let t4 = Tree::tree(4, unsynced_list!(Tree::leaf(5)));
        let t5 = Tree::tree(4, unsynced_list!(Tree::leaf(5)));
        let t6 = Tree::tree(4, unsynced_list!(Tree::leaf(6)));
        assert!(t1 == t2);
        assert!(t1 != t3);
        assert!(t1 != t4);
        assert!(t3 != t4);
        assert!(t3 == Tree::leaf(4));
        assert!(t3 != Tree::leaf(5));
        assert!(t4 == t5);
        assert!(t4 != t6);
        assert!(t6 == t6);
    }
}
