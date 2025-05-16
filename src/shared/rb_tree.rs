use core::borrow::Borrow;

use super::rb_node::{Colour, RBNode};

#[derive(Debug)]
pub struct RBTree<L> {
    root: Option<L>,
}

impl<L> Default for RBTree<L> {
    fn default() -> Self {
        RBTree { root: None }
    }
}

// impl<L: RBLink> Clone for RBTree<L> {
//     fn clone(&self) -> Self {
//         Self {
//             root: self.root.as_ref().map(|link| link.clone()),
//         }
//     }
// }

impl<L> RBTree<L> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}
impl<L: RBNode> RBTree<L> {
    pub fn leaf(element: L::ValueType) -> Self {
        Self {
            root: Some(RBNode::leaf(element)),
        }
    }

    pub fn root(&self) -> Option<&L::ValueType> {
        self.root.as_ref().map(RBNode::get_element)
    }

    pub fn left(&self) -> Self {
        assert!(!self.is_empty());
        Self {
            root: self.root.as_ref().and_then(|node| node.left_cloned()),
        }
    }

    pub fn right(&self) -> Self {
        assert!(!self.is_empty());
        Self {
            root: self.root.as_ref().and_then(|node| node.right_cloned()),
        }
    }

    pub fn contains<Q>(&self, q: &Q) -> bool
    where
        L::ValueType: Borrow<Q> + PartialOrd,
        Q: PartialOrd + ?Sized,
    {
        self.root.as_ref().map_or(false, |node| node.contains(q))
    }

    pub fn get<Q>(&self, q: &Q) -> Option<&L::ValueType>
    where
        L::ValueType: Borrow<Q> + PartialOrd,
        Q: PartialOrd + ?Sized,
    {
        self.root.as_ref().and_then(|node| node.get(q))
    }

    pub fn get_or_default<'a, Q>(&'a self, q: &Q, default: &'a L::ValueType) -> &'a L::ValueType
    where
        L::ValueType: Borrow<Q> + PartialOrd,
        Q: PartialOrd + ?Sized,
    {
        self.get(q).unwrap_or(default)
    }
}

impl<L> RBTree<L>
where
    L: RBNode,
{
    pub fn inserted(&self, x: L::ValueType) -> Self
    where
        L::ValueType: PartialOrd + Clone,
    {
        Self {
            root: node_inserted(&self.root, x),
        }
    }
}

fn node_inserted<L>(node: &Option<L>, x: L::ValueType) -> Option<L>
where
    L: RBNode,
    L::ValueType: PartialOrd + Clone,
{
    let new_link = sorted_insert(node, x);
    paint_link(&new_link, Colour::Black)
}

fn sorted_insert<L>(node: &Option<L>, x: L::ValueType) -> Option<L>
where
    L: RBNode,
    L::ValueType: PartialOrd + Clone,
{
    match node {
        None => Some(L::leaf(x)),
        Some(node) => {
            if x < *node.get_element() {
                balance_node(
                    node.get_colour(),
                    node.get_element().clone(),
                    sorted_insert(&node.left_cloned(), x),
                    node.right_cloned(),
                )
            } else if x > *node.get_element() {
                balance_node(
                    node.get_colour(),
                    node.get_element().clone(),
                    node.left_cloned(),
                    sorted_insert(&node.right_cloned(), x),
                )
            } else {
                Some(node.clone())
            }
        }
    }
}

fn balance_node<L>(c: Colour, x: L::ValueType, left: Option<L>, right: Option<L>) -> Option<L>
where
    L: RBNode,
{
    todo!()
}

fn paint_link<L>(node: &Option<L>, colour: Colour) -> Option<L>
where
    L: RBNode,
{
    todo!()
}
