extern crate rand;

use std::fmt::{Debug, Formatter, Result};
use std::mem;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use rand::distributions::{IndependentSample, Range};

type NodeRef<T> = Rc<RefCell<Node<T>>>;
type ReverseNodeRef<T> = Weak<RefCell<Node<T>>>;

pub struct LinkedList<T: Copy + Debug> {
    length:    usize,
    head:      Option<NodeRef<T>>,
    tail:      Option<ReverseNodeRef<T>>,
}

pub struct Node<T: Copy + Debug> {
    position: usize,
    value:    T,
    previous: Option<ReverseNodeRef<T>>,
    next:     Option<NodeRef<T>>,
    skip:     Option<NodeRef<T>>,
}

impl<T: Copy + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Node value: {:?} position: {} next: {:?} skip: {:?} >", self.value, self.position, self.next, self.skip)
    }
}

impl<T: Copy + Debug> Node<T> {
    fn new(value: T) -> Node<T> {
        increment_node_count();
        Node {
            position: 0,
            value:    value,
            previous: None,
            next:     None,
            skip:     None,
        }
    }

    fn skip_to(&mut self, skip_node: NodeRef<T>, skip_position: usize) {
        if skip_position == self.position {
            return;
        }

        let between = Range::new(0, 100);
        let mut rng = rand::thread_rng();
        match self.skip {
            // if this node already has a skip then we don't override it
            Some(_) => { },
            None => {
                // we want to increase the probablity that we'll
                // add the skip level the further we get away from the current
                // item so we take the difference in position and multiply it
                // by some number, subtract it from 100 and that's our probability
                let offset    = (skip_position - self.position - 1) * 5;
                let threshold = if offset > 100 {
                    0
                } else {
                    100 - offset
                };
                if between.ind_sample(&mut rng) > threshold {
                    self.skip = Some(skip_node.clone());
                } else {
                    match self.previous {
                        None => {},
                        Some (ref mut list) => {
                            let upgraded_list = list.upgrade().unwrap();
                            upgraded_list.borrow_mut().skip_to(skip_node, skip_position);
                        }
                    }
                }
            }
        }
    }
}

impl<T: Copy + Debug> Drop for Node<T> {
    fn drop(&mut self) {
        decrement_node_count();
    }
}

thread_local!(static NODE_COUNT: RefCell<usize> = RefCell::new(0));

fn increment_node_count() {
    NODE_COUNT.with(|c| { *c.borrow_mut() += 1; });
}

fn decrement_node_count() {
    NODE_COUNT.with(|c| { *c.borrow_mut() -= 1; });
}

fn get_node_count() -> usize {
    NODE_COUNT.with(|c| { *c.borrow() })
}

impl<T: Copy + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<List length: {:?} list: {:?}>", self.length, self.head)
    }
}

impl<T: Copy + Debug> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.head {
            Some(ref list) => {
                Some(list.borrow().value)
            },
            None => None
        };

        let mut temp_next = None;
        match self.head {
            Some(ref mut list) => {
                let mut list = list.borrow_mut();
                mem::swap(&mut temp_next, &mut list.next);
            },
            None => {}
        }
        mem::swap(&mut temp_next, &mut self.head);
        match ret {
            Some(_) => self.length -= 1,
            None    => {}
        }
        ret
    }

    fn count(self) -> usize {
        self.length
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<T: Copy + Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            head:   None,
            tail:   None,
        }
    }

    fn append(&mut self, value: T) -> NodeRef<T> {
        self.length += 1;
        let new_tail = self._append(value);
        match self.tail {
            None => {},
            Some(ref list) => {
                let upgraded_list = list.upgrade().unwrap();
                upgraded_list.borrow_mut().skip_to(new_tail.clone(), self.length);
            }
        }
        new_tail
    }

    fn _append(&mut self, value: T) -> NodeRef<T> {
        let node : Node<T> = Node::new(value);
        let node_ref = Rc::new(RefCell::new(node));

        match self.tail {
            Some(ref mut list) => {
                let upgraded_list = list.upgrade().unwrap();
                let new_position = upgraded_list.borrow().position + 1;
                node_ref.borrow_mut().position = new_position;
                node_ref.borrow_mut().previous = Some(list.clone());
                upgraded_list.borrow_mut().next = Some(node_ref.clone());
            },
            None => {
                self.head = Some(node_ref.clone());
            }
        };

        self.tail = Some(Rc::downgrade(&node_ref));

        node_ref
    }

    fn at(&self, position: usize) -> Option<T> {
        match self.head {
            Some(ref list) => {
                let (ret, _) = self._at(position, list, 0);
                ret
            },
            None => None
        }
    }

    fn _at(&self, position: usize, node: &NodeRef<T>, iterations: usize) -> (Option<T>, usize) {
        if position == node.borrow().position {
            (Some(node.borrow().value), iterations)
        } else {
            match node.borrow().skip {
                None => self._next_at(position, node, iterations + 1),
                Some(ref list) => {
                    // this check, if list.position == node.position is really
                    // frustrating but I don't know what's causing it
                    if list.borrow().position == node.borrow().position {
                        self._next_at(position, node, iterations + 1)
                    } else if list.borrow().position <= position {
                        self._at(position, list, iterations + 1)
                    } else {
                        self._next_at(position, node, iterations + 1)
                    }
                }
            }
        }
    }

    fn _next_at(&self, position: usize, node: &NodeRef<T>, iterations: usize) -> (Option<T>, usize) {
        match node.borrow().next {
            Some(ref list) => self._at(position, list, iterations),
            None => (None, iterations)
        }
    }
}

#[cfg(not(test))]
fn main() {
    let mut list = LinkedList::new();
    list.append(0);
    list.append(1);
    println!("{:?}", list);
    println!("{}", list.at(1).unwrap());

    list = LinkedList::new();
    for i in (0 .. 10) {
        list.append(i);
    }

    println!("for loop:");
    for val in list {
        println!("  {}", val);
    }

    list = LinkedList::new();
    for i in (0 .. 10) {
        list.append(i);
    }

    println!("manual iteration");
    let mut iter = list.into_iter();
    loop {
        match iter.next() {
            Some(i) => println!("   {}", i),
            None    => break,
        }
    }

    list = LinkedList::new();
    for i in (0 .. 10) {
        list.append(i);
    }
    println!("{:?}", list);

    list = LinkedList::new();
    for i in (0 .. 100) {
        list.append(i);
    }
    list.at(100);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_increments_length() {
        let mut list = LinkedList::new();
        list.append(0);
        assert_eq!(list.length, 1);
    }

    #[test]
    fn insert_at_for_one_value() {
        let mut list = LinkedList::new();
        list.append(0);
        assert_eq!(list.at(0), Some(0));
    }

    #[test]
    fn insert_at_for_two_values() {
        let mut list = LinkedList::new();
        list.append(0);
        list.append(1);
        assert_eq!(list.at(0), Some(0));
        assert_eq!(list.at(1), Some(1));
    }

    #[test]
    fn many_inserts() {
        let mut list = LinkedList::new();
        for i in (0 .. 100) {
            list.append(i);
        }

        for i in (0 .. 100) {
            assert_eq!(list.at(i), Some(i));
        }
    }

    #[test]
    fn many_inserts_and_iteration() {
        let mut list = LinkedList::new();
        for i in (0 .. 100) {
            list.append(i);
        }

        let mut i = 0;
        for item in list.into_iter() {
            assert_eq!(item, i);
            i += 1;
        }
    }

    #[test]
    fn many_inserts_lookup_iterations_should_be_less_than_items() {
        let mut list = LinkedList::new();
        let head = list.append(0);
        for i in (1 .. 1000) {
            list.append(i);
        }

        let (result, it) = list._at(999, &head, 0);
        assert!(it < 999);
        assert_eq!(result.unwrap(), 999);
    }

    use super::get_node_count;

    #[test]
    fn nodes_should_not_leak() {
        let count_before = get_node_count();
        {
            let mut list = LinkedList::new();
            for i in (0 .. 7) {
                list.append(i);
            }

            assert_eq!(get_node_count(), count_before + 7);
        }

        assert_eq!(get_node_count(), count_before);
    }
}
