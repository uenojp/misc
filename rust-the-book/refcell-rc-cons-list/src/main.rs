use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

fn main() {
    let list = List::from(&[1, 2, 3, 4][..]);
    println!("{list}");

    let list = List::from(&[42, 44][..]);
    println!("{list}");
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            List::Cons(head, tail) => write!(f, "{} -> {}", head, tail.borrow()),
            List::Nil => write!(f, "Nil"),
        }
    }
}

impl std::convert::From<&[i32]> for List {
    fn from(slice: &[i32]) -> Self {
        let mut list = List::Nil;
        for element in slice.iter().rev() {
            list = List::Cons(*element, RefCell::new(Rc::new(list)));
        }
        list
    }
}
