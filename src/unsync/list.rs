use alloc::rc::Rc;
use core::fmt::Debug;

use crate::shared::{self, link::Link};

pub type List<T> = shared::list::List<UnsyncLink<T>>;
pub use crate::shared::list::{filter, fmap, foldl, foldr, mreturn};

type UnsyncLink<T> = Rc<Node<T>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Link for Rc<Node<T>> {
    type ValueType = T;
    fn from_value(element: Self::ValueType) -> Self {
        Rc::new(Node {
            element,
            next: None,
        })
    }
    fn cons(element: Self::ValueType, next: Option<Self>) -> Self {
        Rc::new(Node { element, next })
    }
    fn clone(&self) -> Self {
        <Self as Clone>::clone(&self)
    }
    fn get_element(&self) -> &Self::ValueType {
        &self.element
    }
    fn next_cloned(&self) -> Option<Self> {
        self.next.as_ref().map(|node| <Self as Clone>::clone(node))
    }
    fn next_ref(&self) -> Option<&Self> {
        self.next.as_ref().map(|node| node)
    }
    fn link_ref(&self) -> &Self {
        self
    }
}

#[macro_export]
macro_rules! unsynced_list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_list = List::default();
            $(
                temp_list = temp_list.pushed_front($x);
             )*
            temp_list
        }
    };
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        let mut next = self.next.take();
        while let Some(node) = next {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                next = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty() {
        let list = List::<i32>::new();

        assert!(list.is_empty());
        assert_eq!(list.front(), None);
    }

    #[test]
    fn create_cons() {
        let list: List<i32> = List::cons(3, &List::new());

        assert_eq!(list.front(), Some(&3));

        assert!(!list.is_empty());
    }

    #[test]
    #[should_panic]
    fn popped_front_on_empty_list_panics() {
        let list = List::<i32>::new();
        list.popped_front();
    }
    #[test]
    fn pushed_front_creates_new_longer_list() {
        let l1 = List::new();
        let l2 = l1.pushed_front(6.7);

        assert!(l1.is_empty());
        assert_eq!(l1.front(), None);

        assert!(!l2.is_empty());
        assert_eq!(l2.front(), Some(&6.7));
    }

    #[test]
    fn popped_front_returns_tail() {
        let l1 = List::new();
        let l2 = List::cons(3, &l1);
        let l3 = List::cons(4, &l2);

        assert_eq!(l3.front(), Some(&4));
        let l4 = l3.popped_front();
        assert_eq!(l4.front(), Some(&3));
    }

    #[test]
    fn list_macro_creates_list_in_reversed_order() {
        let l1 = unsynced_list!(1);
        assert_eq!(l1.front(), Some(&1));
        assert!(l1.popped_front().is_empty());

        let l2 = unsynced_list!(1, 2);
        assert_eq!(l2.front(), Some(&2));
        assert_eq!(l2.popped_front().front(), Some(&1));
        assert!(l2.popped_front().popped_front().is_empty());
    }

    mod iter {
        use super::*;
        #[test]
        fn empty_list() {
            let l0 = List::<i32>::empty();
            let mut l0_iter = l0.iter();
            assert_eq!(l0_iter.next(), None);
        }

        #[test]
        fn singelton_list() {
            let l0 = List::empty();
            let l1 = List::cons(1, &l0);
            let mut l_iter = l1.iter();
            assert_eq!(l_iter.next(), Some(&1));
        }
    }
    #[test]
    fn filter_creates_new_list_with_fn_predicate() {
        fn even(v: &i32) -> bool {
            v % 2 == 0
        }

        let list = unsynced_list!(4, 3, 2, 1);

        let evens = filter(even, &list);

        assert_eq!(evens, unsynced_list!(4, 2));
    }

    #[test]
    fn test_partial_eq() {
        let l1 = unsynced_list!(1, 2, 3);

        assert_eq!(l1, l1);
        assert_eq!(List::<i32>::new(), List::<i32>::new());
        assert_eq!(unsynced_list!(5, 7, 0), unsynced_list!(5, 7, 0));
    }

    #[test]
    fn fmap_creates_new_list_with_fn_function() {
        fn double(v: &i32) -> i32 {
            v * 2
        }

        let list = unsynced_list!(4, 3, 2, 1);

        let doubles = fmap(double, &list);

        assert_eq!(doubles, unsynced_list!(2, 4, 6, 8));
    }

    #[test]
    fn sum_w_foldl_and_foldr_are_equal() {
        fn sum(a: i32, b: &i32) -> i32 {
            a + b
        }

        let list = unsynced_list!(4, 3, 2, 1);

        assert_eq!(foldl(sum, 0, &list), foldr(|a, b| a + b, 0, &list));
    }

    #[test]
    fn mreturn_creates_list() {
        let list = mreturn(3);

        assert_eq!(list, unsynced_list!(3));
    }
}
