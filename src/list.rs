use std::rc::Rc;

// pub enum List<T> {
//      Empty,
//      Head(Rc<Node<T>>),
// }

pub struct List<T> {
    head: Rc<Node<T>>,
}

pub enum Node<T> {
    Empty,
    Link(T, Rc<Node<T>>),
}

impl<T> List<T> {
    pub fn empty() -> List<T> {
        List { head: Rc::new( Node::<T>::Empty ) }
    }

    pub fn cons(head: T, tail: &List<T>) -> List<T> {
        List { 
            head: Rc::new( 
                Node::Link(
                    head, 
                    Rc::clone(&tail.head) 
                )
            ) 
        }
    }

    fn from_node(tail: &Rc<Node<T>>) -> List<T> {
        List { head: Rc::clone(tail) }
    }
    pub fn from_value(value: T) -> List<T> {
        List { head: Rc::new( Node::Link(value, Rc::new( Node::Empty )) ) }
    }

    pub fn front(&self) -> Option<&T> {
        match &*self.head {
            Node::Empty => None,
            Node::Link(head, _tail) => Some(&head),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &*self.head {
            Node::Empty => true,
            _ => false,
        }
    }

    pub fn pop_front(&self) -> List<T> {
        match &*self.head {
            Node::Empty => panic!("You can't pop an empty list!"),
            Node::Link(_head, tail) => List::from_node(tail),
        }
    }

    pub fn push_front(&self, value: T) -> List<T> {
        List::cons(value, self)
    }
}

pub fn filter<T: Copy>(
    p: impl FnOnce(&T) -> bool + Copy, 
    list: &List<T>
) -> List<T> {
    match list.front() {
        Some(head) => {
            let tail = filter(p, &list.pop_front());
            if p(head) {
                List::cons(*head, &tail)
            } else {
                tail
            }
        },
        None => List::empty()
        
    }
} 

pub fn fmap<U, T>(f: impl FnOnce(&T) -> U + Copy, list: &List<T>) -> List<U> {
    match list.front() {
        None => List::<U>::empty(),
        Some(head) => List::cons(f(head), &fmap(f, &list.pop_front()))
    }
}

pub fn foldl<U, T>(f: impl FnOnce(U, &T) -> U + Copy, acc: U, list: &List<T>) -> U {
    match list.front() {
        None => acc,
        Some(head) => foldl(f, f(acc, head), &list.pop_front())
    }
}

pub fn foldr<U, T>(f: impl FnOnce(&T, U) -> U + Copy, acc: U, list: &List<T>) -> U {
    match list.front() {
        None => acc,
        Some(head) => f(head, foldr(f, acc, &list.pop_front()))
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::list::Node::{Empty, Link};

    #[test]
    fn create_empty() {
        let list = List::<i32>::empty();

        match *list.head {
            Empty => assert!(true),
            _ => assert!(false),
        };
        assert!(list.is_empty());
    }

    #[test]
    fn create_cons() {

        let list = List {
            head: Rc::new(
                Node::Link(3, Rc::new(Node::Empty))
            )
        };

        assert_eq!(list.front(), Some(&3));
        match &*list.head {
            Link(head, tail) => {
                assert_eq!(head, &3);
            },
            _ => panic!("Should not be here.")
        };
        assert!(!list.is_empty());
    }

    #[test]
    fn push_front_creates_new_longer_list() {
        let l1 = List::empty();
        let l2 = l1.push_front(6.7);

        assert!(l1.is_empty());
        assert_eq!(l1.front(), None);

        assert!(!l2.is_empty());
        assert_eq!(l2.front(), Some(&6.7));
    }

    #[test]
    fn pop_front_returns_tail() {
        let l1 = List::empty();
        let l2 = List::cons(3, &l1);
        let l3 = List::cons(4, &l2);

        assert_eq!(l3.front(), Some(&4));
        let l4 = l3.pop_front();
        assert_eq!(l4.front(), Some(&3));
    }

    #[test]
    fn filter_creates_new_list_with_fn_predicate() {
        fn even(v: &i32) -> bool {
            v % 2 == 0
        }

        let list = List::cons(
            4,
            &List::cons(
                3,
                &List::cons(
                    2,
                    &List::cons(
                        1,
                        &List::empty()
                    )
                )
            )
        );

        let evens = filter(even, &list);

        assert_eq!(evens.front(), Some(&4));
        assert_eq!(evens.pop_front().front(), Some(&2));
        assert_eq!(evens.pop_front().pop_front().front(), None);

    }

    #[test]
    fn fmap_creates_new_list_with_fn_function() {
        fn double(v: &i32) -> i32 {
            v * 2
        }

        let list = List::cons(
            4,
            &List::cons(
                3,
                &List::cons(
                    2,
                    &List::cons(
                        1,
                        &List::empty()
                    )
                )
            )
        );

        let doubles = fmap(double, &list);

        assert_eq!(doubles.front(), Some(&8));
        assert_eq!(doubles.pop_front().front(), Some(&6));
        assert_eq!(doubles.pop_front().pop_front().front(), Some(&4));

    }

    #[test]
    fn sum_w_foldl_and_foldr_are_equal() {
        fn sum(a: i32, b: &i32) -> i32 {
            a + b
        }

        let list = List::cons(
            4,
            &List::cons(
                3,
                &List::cons(
                    2,
                    &List::cons(
                        1,
                        &List::empty()
                    )
                )
            )
        );

        assert_eq!(
            foldl(sum, 0, &list), 
            foldr(|a, b| a+b, 0, &list)
        );

    }
}
