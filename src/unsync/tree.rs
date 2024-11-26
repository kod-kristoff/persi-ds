use crate::unsync::list::List;
use alloc::rc::Rc;

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

impl<T: Clone> Clone for TreeNode<T> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            children: self.children.clone(),
        }
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

    #[test]
    fn tree_is_clone() {
        let t1 = Tree::<&str>::new();
        let t2 = Tree::leaf("a");
        let t3 = Tree::tree("b", List::from_value(t2.clone()));

        assert_eq!(t1, t1.clone());
        assert_eq!(t2, t2.clone());
        assert_eq!(t3, t3.clone());
    }

    mod non_clonable {
        use alloc::{boxed::Box, string::String};
        use super::*;

        #[derive(Debug, PartialEq)]
        struct NoClone {
            pub v: Box<String>,
        }

        #[test]
        fn leaf_tree_can_be_cloned() {
            let v1 = NoClone {
                v: Box::new(String::from("r")),
            };
            let t1 = Tree::leaf(v1);
            let t1_clone = t1.clone();
            assert_eq!(t1, t1_clone);
        }

        #[test]
        fn tree_can_be_cloned() {
            let v1 = NoClone {
                v: Box::new(String::from("r")),
            };
            let v2 = NoClone {
                v: Box::new(String::from("r")),
            };
            let t1 = Tree::leaf(v1);
            let t2 = Tree::tree(v2, List::from_value(t1.clone()));
            let t1_clone = t1.clone();
            assert_eq!(t1, t1_clone);
            let t2_clone = t2.clone();
            assert_eq!(t2, t2_clone);
        }
    }
}
