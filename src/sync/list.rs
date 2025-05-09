use alloc::sync::Arc;
use core::fmt::Debug;

use crate::shared;
use crate::shared::link::Link;

pub type List<T> = shared::list::List<SyncLink<T>>;

pub use crate::shared::list::{filter, fmap, foldl, foldr, mreturn};

type SyncLink<T> = Arc<Node<T>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Option<SyncLink<T>>,
}

impl<T> Link for Arc<Node<T>> {
    type ValueType = T;

    fn from_value(element: Self::ValueType) -> Self {
        Arc::new(Node {
            element,
            next: None,
        })
    }

    fn cons(element: Self::ValueType, next: Option<Self>) -> Self {
        Arc::new(Node { element, next })
    }

    fn clone(&self) -> Self {
        <Self as Clone>::clone(&self)
    }

    fn link_ref(&self) -> &Self {
        self
    }

    fn next_ref(&self) -> Option<&Self> {
        self.next.as_ref()
    }

    fn get_element(&self) -> &Self::ValueType {
        &self.element
    }

    fn next_cloned(&self) -> Option<Self> {
        self.next.as_ref().map(|node| <Self as Clone>::clone(node))
    }
}

#[macro_export]
macro_rules! synced_list {
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
            if let Ok(mut node) = Arc::try_unwrap(node) {
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
        let list = List::cons(3, &List::empty());

        assert_eq!(list.front(), Some(&3));

        assert!(!list.is_empty());
    }

    #[test]
    fn pushed_front_creates_new_longer_list() {
        let l1 = List::empty();
        let l2 = l1.pushed_front(6.7);

        assert!(l1.is_empty());
        assert_eq!(l1.front(), None);

        assert!(!l2.is_empty());
        assert_eq!(l2.front(), Some(&6.7));
    }

    #[test]
    fn popped_front_returns_tail() {
        let l1 = List::empty();
        let l2 = List::cons(3, &l1);
        let l3 = List::cons(4, &l2);

        assert_eq!(l3.front(), Some(&4));
        let l4 = l3.popped_front();
        assert_eq!(l4.front(), Some(&3));
    }

    #[test]
    fn list_macro_creates_list_in_reversed_order() {
        let l1 = synced_list!(1);
        assert_eq!(l1.front(), Some(&1));
        assert!(l1.popped_front().is_empty());

        let l2 = synced_list!(1, 2);
        assert_eq!(l2.front(), Some(&2));
        assert_eq!(l2.popped_front().front(), Some(&1));
        assert!(l2.popped_front().popped_front().is_empty());
    }

    #[test]
    fn filter_creates_new_list_with_fn_predicate() {
        fn even(v: &i32) -> bool {
            v % 2 == 0
        }

        let list = synced_list!(4, 3, 2, 1);

        let evens = filter(even, &list);

        assert_eq!(evens, synced_list!(4, 2));
    }

    #[test]
    fn test_partial_eq() {
        let l1 = synced_list!(1, 2, 3);

        assert_eq!(l1, l1);
        assert_eq!(List::<i32>::empty(), List::<i32>::empty());
        assert_eq!(synced_list!(5, 7, 0), synced_list!(5, 7, 0));
    }

    #[test]
    fn fmap_creates_new_list_with_fn_function() {
        fn double(v: &i32) -> i32 {
            v * 2
        }

        let list = synced_list!(4, 3, 2, 1);

        let doubles = fmap(double, &list);

        assert_eq!(doubles, synced_list!(2, 4, 6, 8));
    }

    #[test]
    fn sum_w_foldl_and_foldr_are_equal() {
        fn sum(a: i32, b: &i32) -> i32 {
            a + b
        }

        let list = synced_list!(4, 3, 2, 1);

        assert_eq!(foldl(sum, 0, &list), foldr(|a, b| a + b, 0, &list));
    }

    #[test]
    fn mreturn_creates_list() {
        let list = mreturn(3);

        assert_eq!(list, synced_list!(3));
    }

    mod no_copy {

        use shared::list::concat_all;

        use super::*;

        #[derive(Debug, PartialEq, Clone)]
        struct NoCopy(i32);

        #[test]
        fn new_creates_empty_list() {
            let lst = List::<NoCopy>::new();

            assert!(lst.is_empty());
        }

        #[test]
        fn concat_all_concats() {
            let lst1 = List::from_value(NoCopy(1));
            let lst2 = List::from_value(NoCopy(2));
            let lst_all = synced_list!(lst1, lst2);

            assert_eq!(
                concat_all(&lst_all),
                List::cons(NoCopy(1), &List::from_value(NoCopy(2)))
            )
        }

        #[test]
        fn sum_w_foldl_and_foldr_are_equal() {
            fn sum(a: NoCopy, b: &NoCopy) -> NoCopy {
                NoCopy(a.0 + b.0)
            }

            let list = synced_list!(NoCopy(4), NoCopy(3), NoCopy(2), NoCopy(1));

            assert_eq!(
                foldl(sum, NoCopy(0), &list),
                foldr(|a, b| NoCopy(a.0 + b.0), NoCopy(0), &list)
            );
        }

        #[test]
        fn sub_w_foldl_and_foldr_are_unequal() {
            fn sub(a: NoCopy, b: &NoCopy) -> NoCopy {
                NoCopy(a.0 - b.0)
            }

            let list = synced_list!(NoCopy(1), NoCopy(1), NoCopy(1));

            assert_eq!(foldl(sub, NoCopy(0), &list), NoCopy(-3));
            assert_eq!(foldr(|a, b| NoCopy(a.0 - b.0), NoCopy(0), &list), NoCopy(1));
        }
    }
}
