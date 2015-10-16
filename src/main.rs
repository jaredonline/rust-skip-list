use std::fmt::{Debug, Formatter, Result};

pub struct LinkedList<T> {
    length: usize,
    value:  Option<T>,
    next:   Option<Box<LinkedList<T>>>,
}

impl<T: Copy + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<List length: {:?} value: {:?}>", self.length, self.value)
    }
}

impl<T: Copy> LinkedList<T> {
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

    fn at(&mut self, position: usize) -> T {
        if position >= self.length {
            panic!("index: {} is out of bounds for List of size {}", position, self.length);
        }

        match position {
            0 => self.value.unwrap(),
            _ => {
                match self.next {
                    Some(ref mut list) => list.at(position - 1),
                    None => panic!("no value at that index")
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
    println!("{}", list.at(1));

    list.at(2);
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
        assert!(list.at(0) == 0);
    }

    #[test]
    fn insert_at_for_two_values() {
        let mut list = LinkedList::new();
        list.append(0);
        list.append(1);
        assert!(list.at(0) == 0);
        assert!(list.at(1) == 1);
    }

    #[test]
    fn many_inserts() {
        let mut list = LinkedList::new();
        for i in (0 .. 100) {
            list.append(i);
        }

        for i in (0 .. 100) {
            assert!(list.at(i) == i);
        }
    }
}
