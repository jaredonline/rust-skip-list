extern crate rand;

use std::fmt::{Debug, Formatter, Result};
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use rand::distributions::{IndependentSample, Range};

type Next<T> = Rc<RefCell<LinkedList<T>>>;

pub struct LinkedList<T: Copy + Debug> {
    length:    usize,
    value:     Option<T>,
    next:      Option<Next<T>>,
    next_skip: Option<Next<T>>,
    position:  usize,
}

impl<T: Copy + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<List position: {} length: {:?} value: {:?} skip: {:?} list: {:?}>", self.position, self.length, self.value, self.next_skip, self.next)
    }
}

impl<T: Copy + Debug> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.value;
        let mut temp_next = None;
        let mut val       = None;
        match self.next {
            Some(ref mut list) => {
                let mut list = list.borrow_mut();
                val = list.value;
                mem::swap(&mut temp_next, &mut list.next);
            },
            None => {}
        }
        self.value = val;
        mem::swap(&mut temp_next, &mut self.next);
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
            length:    0,
            value:     None,
            next:      None,
            next_skip: None,
            position:  0,
        }
    }

    fn append(&mut self, value: T) -> Next<T> {
        self.length += 1;
        let next = match self.next {
            Some(ref mut list) => {
                list.borrow_mut().append(value)
            },
            None => {
                let mut list : LinkedList<T> = LinkedList::new();
                list.position = self.position + 1;
                let next = Rc::new(RefCell::new(list));
                self.value = Some(value);
                self.next  = Some(next.clone());
                next
            }
        };

        let between = Range::new(0, 100);
        let mut rng = rand::thread_rng();
        match self.next_skip {
            Some(_) => { },
            None => {
                // if oer 50% set next_skip to Some(next.cone());
                if between.ind_sample(&mut rng) > 50 {
                    self.next_skip = Some(next.clone());
                }
            }
        }

        next
    }

    fn at(&self, position: usize) -> Option<T> {
        match position {
            0 => self.value,
            _ => {
                match self.next_skip {
                    Some(ref list) => {
                        let list = list.borrow();
                        if position > list.position {
                            self.skip_at(position - list.position + self.position)
                        } else {
                            self.next_at(position - 1)
                        }
                    },
                    None => {
                        self.next_at(position - 1)
                    }
                }
            }
        }
    }

    fn skip_at(&self, position: usize) -> Option<T> {
        match self.next_skip {
            Some(ref list) => list.borrow().at(position),
            None => None
        }
    }

    fn next_at(&self, position: usize) -> Option<T> {
        match self.next {
            Some(ref list) => list.borrow().at(position),
            None => None
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
}
