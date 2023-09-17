use itertools::{EitherOrBoth, Itertools};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
    }
}
impl<T> List<T> {
    pub fn empty() -> List<T> {
        Self::default()
    }

    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn cons(element: T, tail: &List<T>) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                element,
                next: tail.head.clone(),
            })),
        }
    }

    pub fn from_value(element: T) -> List<T> {
        List {
            head: Some(Arc::new(Node {
                element,
                next: None,
            })),
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn popped_front(&self) -> List<T> {
        if self.head.is_none() {
            panic!("You can't pop an empty list!");
        }
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn tail(&self) -> List<T> {
        self.popped_front()
    }

    pub fn head_tail(&self) -> (Option<T>, List<T>)
    where
        T: Clone,
    {
        (self.front().cloned(), self.tail())
    }

    pub fn head_tail_ref(&self) -> (Option<&T>, List<T>) {
        (self.front(), self.tail())
    }

    pub fn pushed_front(&self, value: T) -> List<T> {
        List::cons(value, self)
    }

    pub fn reversed(&self) -> Self
    where
        T: Clone,
    {
        reverse(self.clone())
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

#[macro_export]
macro_rules! synced_list {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_list = List::empty();
            $(
                temp_list = temp_list.pushed_front($x);
             )*
            temp_list
        }
    };
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            head: self.head.clone(),
        }
    }
}

// impl<T: fmt::Debug> fmt::Debug for List<T> {}
impl<T> fmt::Display for List<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "list [")?;
        for x in self {
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

impl<T> PartialEq for List<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        match (self.is_empty(), other.is_empty()) {
            (true, true) => return true,
            (true, false) => return false,
            (false, true) => return false,
            _ => {}
        };
        match (self.head_tail_ref(), other.head_tail_ref()) {
            ((Some(self_head), self_tail), (Some(other_head), other_tail)) => {
                self_head == other_head && self_tail == other_tail
            }
            _ => false,
        }
    }
}

pub fn filter<T: Clone>(p: impl FnOnce(&T) -> bool + Copy, list: &List<T>) -> List<T> {
    match list.front() {
        Some(head) => {
            let tail = filter(p, &list.popped_front());
            if p(head) {
                List::cons(head.clone(), &tail)
            } else {
                tail
            }
        }
        None => List::empty(),
    }
}

pub fn reverse<T: Clone>(list: List<T>) -> List<T> {
    foldl(
        |acc: List<T>, v: &T| List::cons(v.clone(), &acc),
        List::empty(),
        list,
    )
}

pub fn fmap<U, T, F>(mut f: F, list: &List<T>) -> List<U>
where
    F: FnMut(&T) -> U,
{
    let mut result = List::<U>::empty();
    for x in list {
        result = result.pushed_front(f(&x));
    }
    result
}

pub fn foldl<U, T, F>(mut f: F, mut acc: U, mut list: List<T>) -> U
where
    F: FnMut(U, &T) -> U,
{
    loop {
        match list.front() {
            None => break,
            Some(head) => {
                acc = f(acc, head);
                list = list.popped_front()
            }
        }
    }
    acc
}

pub fn foldr<U, T, F>(f: F, acc: U, list: &List<T>) -> U
where
    F: FnOnce(&T, U) -> U + Copy,
    T: Clone,
{
    match list.front() {
        None => acc,
        Some(head) => f(head, foldr(f, acc, &list.popped_front())),
    }
}

pub fn for_each<T>(list: &List<T>, mut f: impl FnMut(&T)) {
    let mut node = &list.head;
    loop {
        match node {
            None => break,
            Some(head) => {
                f(&head.element);
                node = &head.next;
            }
        };
    }
}

pub fn concat<T: Clone>(a: &List<T>, b: &List<T>) -> List<T> {
    match a.front() {
        None => b.clone(),
        Some(head) => List::cons(head.clone(), &concat(&a.popped_front(), b)),
    }
}

pub fn concat_all<T: Clone>(xss: &List<List<T>>) -> List<T> {
    // let result = foldr(|xs, acc| concat(xs, &acc), List::<T>::empty(), xss);
    let mut result = List::<T>::empty();
    for xs in xss {
        for x in xs {
            result = result.pushed_front(x.clone());
        }
    }
    result
}

// List Monad
pub fn mreturn<T>(t: T) -> List<T> {
    List::from_value(t)
}

pub fn mbind<A, B, F>(list: &List<A>, k: F) -> List<B>
where
    F: Fn(&A) -> List<B>,
    B: Clone,
{
    let list_list = fmap(k, list);
    concat_all(&list_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty() {
        let list = List::<i32>::empty();

        match &list.head {
            None => assert!(true),
            _ => assert!(false),
        };
        assert!(list.is_empty());
        assert_eq!(list.front(), None);
    }

    #[test]
    fn create_cons() {
        let list = List::cons(3, &List::empty());

        assert_eq!(list.front(), Some(&3));
        match &list.head {
            Some(node) => {
                assert_eq!(&node.element, &3);
                assert!(node.next.is_none());
            }
            _ => panic!("Should not be here."),
        };
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

        assert_eq!(foldl(sum, 0, list.clone()), foldr(|a, b| a + b, 0, &list));
    }

    #[test]
    fn mreturn_creates_list() {
        let list = mreturn(3);

        assert_eq!(list, synced_list!(3));
    }

    mod no_copy {

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
                foldl(sum, NoCopy(0), list.clone()),
                foldr(|a, b| NoCopy(a.0 + b.0), NoCopy(0), &list)
            );
        }

        #[test]
        fn sub_w_foldl_and_foldr_are_unequal() {
            fn sub(a: NoCopy, b: &NoCopy) -> NoCopy {
                NoCopy(a.0 - b.0)
            }

            let list = synced_list!(NoCopy(1), NoCopy(1), NoCopy(1));

            assert_eq!(foldl(sub, NoCopy(0), list.clone()), NoCopy(-3));
            assert_eq!(foldr(|a, b| NoCopy(a.0 - b.0), NoCopy(0), &list), NoCopy(1));
        }
    }
}
