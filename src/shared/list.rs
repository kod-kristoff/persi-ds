use core::{borrow::Borrow, fmt::Debug};

use itertools::{EitherOrBoth, Itertools};

use super::link::Link;

#[derive(Debug)]
pub struct List<L> {
    head: Option<L>,
}

impl<L> Default for List<L> {
    fn default() -> Self {
        List { head: None }
    }
}

impl<L: Link> Clone for List<L> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.as_ref().map(|link| link.clone()),
        }
    }
}

impl<L: Link> List<L> {
    /// Creates an empty `List`.
    ///
    /// # Examples
    /// ```
    /// use persi_ds::unsync::List;
    /// let list: List<u32> = List::new();
    /// ```
    pub fn new() -> List<L> {
        List::default()
    }

    /// Creates an empty `List`.
    ///
    /// # Examples
    /// ```
    /// use persi_ds::unsync::List;
    /// let list: List<u32> = List::empty();
    /// ```
    pub fn empty() -> List<L> {
        List::default()
    }

    /// Creates a list with the element given as head
    /// and the provided list as tail.
    ///
    /// Complexity: O(1)
    ///
    /// # Examples
    /// ```
    /// use persi_ds::unsync::List;
    ///
    /// let list = List::cons(1, &List::new());
    /// let list = List::cons(1, List::new());
    /// ```
    pub fn cons<N>(element: L::ValueType, tail: N) -> List<L>
    where
        N: AsRef<List<L>>,
    {
        List {
            head: Some(Link::cons(
                element,
                tail.as_ref().head.as_ref().map(|link| link.clone()),
            )),
        }
    }

    pub fn from_value(element: L::ValueType) -> Self {
        List {
            head: Some(Link::from_value(element)),
        }
    }

    /// Provides a reference to the front element, or
    /// `None` if the list is empty..
    ///
    /// # Examples
    ///
    /// ```
    /// use persi_ds::unsync::List;
    ///
    /// let l1 = List::<i32>::new();
    /// assert_eq!(l1.front(), None);
    ///
    /// let l2 = List::cons(5, &l1);
    /// assert_eq!(l2.front(), Some(&5));
    /// ```
    pub fn front(&self) -> Option<&L::ValueType> {
        self.head.as_ref().map(Link::get_element)
    }

    /// Returns `true` if this `List` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use persi_ds::unsync::List;
    ///
    /// let l1 = List::<i32>::new();
    /// assert!(l1.is_empty());
    ///
    /// let l2 = List::cons(5, &l1);
    /// assert!(!l2.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the tail of the list.
    ///
    /// # Panics
    ///
    /// This method panics when called on an empty list.
    ///
    /// # Examples
    ///
    /// ```
    /// use persi_ds::unsync::List;
    ///
    /// let l1 = List::<i32>::new();
    /// assert!(l1.is_empty());
    ///
    /// let l2 = List::cons(5, &l1);
    /// assert!(!l2.is_empty());
    /// assert_eq!(l2.popped_front(), l1);
    /// ```
    pub fn popped_front(&self) -> List<L> {
        if self.head.is_none() {
            panic!("You can't pop an empty list!");
        }
        List {
            head: self.head.as_ref().and_then(Link::next_cloned),
        }
    }

    //     pub fn tail(&self) -> List<T> {
    //         self.popped_front()
    //     }

    //     pub fn head_tail(&self) -> (Option<T>, List<T>)
    //     where
    //         T: Clone,
    //     {
    //         (self.front().cloned(), self.tail())
    //     }

    pub fn pushed_front(&self, value: L::ValueType) -> Self {
        List::cons(value, self.clone())
    }

    //     pub fn reversed(&self) -> Self
    //     where
    //         T: Clone,
    //     {
    //         reverse(self)
    //     }

    pub fn iter(&self) -> Iter<'_, L> {
        Iter {
            next: self.head.as_ref().map(Link::link_ref),
            // next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<L> AsRef<List<L>> for List<L> {
    fn as_ref(&self) -> &List<L> {
        &self
    }
}

pub struct Iter<'a, L> {
    next: Option<&'a L>,
}

impl<'a, L: Link> Iterator for Iter<'a, L> {
    type Item = &'a L::ValueType;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next_ref();
            node.get_element()
        })
    }
}

impl<'a, L: Link> IntoIterator for &'a List<L> {
    type Item = &'a L::ValueType;
    type IntoIter = Iter<'a, L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<L> PartialEq for List<L>
where
    L: Link,
    L::ValueType: PartialEq, // T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip_longest(other.iter())
            .all(|x| matches!(x, EitherOrBoth::Both(a, b) if a == b))
    }
}

pub fn filter<T, L>(p: impl FnOnce(&T) -> bool + Copy, list: &List<L>) -> List<L>
where
    L: Link<ValueType = T>,
    T: Copy,
{
    match list.front() {
        Some(head) => {
            let tail = filter(p, &list.popped_front());
            if p(head) {
                List::cons(*head, tail)
            } else {
                tail
            }
        }
        None => List::new(),
    }
}

pub fn fmap<L1, L2>(f: impl Fn(&L1::ValueType) -> L2::ValueType, list: &List<L1>) -> List<L2>
where
    L1: Link,
    L2: Link,
{
    let mut result = List::default();
    for x in list {
        result = result.pushed_front(f(&x));
    }
    result
}

pub fn foldl<L, U>(f: impl FnOnce(U, &L::ValueType) -> U + Copy, acc: U, list: &List<L>) -> U
where
    L: Link,
{
    match list.front() {
        None => acc,
        Some(head) => foldl(f, f(acc, head), &list.popped_front()),
    }
}

pub fn foldr<L, U>(f: impl FnOnce(&L::ValueType, U) -> U + Copy, acc: U, list: &List<L>) -> U
where
    L: Link,
{
    match list.front() {
        None => acc,
        Some(head) => f(head, foldr(f, acc, &list.popped_front())),
    }
}

pub fn concat<L>(a: &List<L>, b: &List<L>) -> List<L>
where
    L: Link,
    L::ValueType: Clone,
{
    match a.front() {
        None => b.clone(),
        Some(head) => List::cons(head.clone(), &concat(&a.popped_front(), b)),
    }
}

pub fn concat_all<L1, L2, T>(xss: &List<L1>) -> List<L2>
where
    L1: Link<ValueType = List<L2>>,
    L2: Link<ValueType = T>,
    T: Clone,
{
    // let result = foldr(|xs, acc| concat(xs, &acc), List::default(), xss);
    let mut result = List::default();
    for xs in xss {
        for x in xs {
            result = result.pushed_front(x.clone());
        }
    }
    result
}

// List Monad
pub fn mreturn<L: Link>(t: L::ValueType) -> List<L> {
    List::cons(t, &List::new())
}

pub fn mbind<L1, L2, L3>(list: &List<L1>, k: impl Fn(&L1::ValueType) -> List<L2> + Copy) -> List<L2>
where
    L1: Link,
    L2: Link,
    L3: Link<ValueType = List<L2>>,
    L2::ValueType: Clone,
{
    let list_list: List<L3> = fmap(k, list);
    concat_all(&list_list)
}
