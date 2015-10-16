use std::fmt::{Debug, Formatter, Result};
use std::mem;

pub struct LinkedList<T: Copy + Debug> {
    length: usize,
    value:  Option<T>,
    next:   Option<Box<LinkedList<T>>>,
}

impl<T: Copy + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<List length: {:?} value: {:?}>", self.length, self.value)
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
            length: 0,
            value:  None,
            next:   None,
        }
    }

    fn append(&mut self, value: T) {
        match self.next {
            Some(ref mut list) => {
                list.append(value);
            },
            None => {
                self.value = Some(value);
                self.next  = Some(Box::new(LinkedList::new()));
            }
        }
        self.length += 1;
    }

    fn at(&mut self, position: usize) -> Option<T> {
        match position {
            0 => self.value,
            _ => {
                match self.next {
                    Some(ref mut list) => list.at(position - 1),
                    None => None
                }
            }
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_increments_length() {
        let mut list = LinkedList::new();
        list.append(0);
        assert!(list.length == 1);
    }

    #[test]
    fn insert_at_for_one_value() {
        let mut list = LinkedList::new();
        list.append(0);
        assert!(list.at(0) == Some(0));
    }

    #[test]
    fn insert_at_for_two_values() {
        let mut list = LinkedList::new();
        list.append(0);
        list.append(1);
        assert!(list.at(0) == Some(0));
        assert!(list.at(1) == Some(1));
    }

    #[test]
    fn many_inserts() {
        let mut list = LinkedList::new();
        for i in (0 .. 100) {
            list.append(i);
        }

        for i in (0 .. 100) {
            assert!(list.at(i) == Some(i));
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
            assert!(item == i);
            i += 1;
        }
    }
}
