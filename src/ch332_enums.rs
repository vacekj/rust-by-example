use crate::ch332_enums::List::{Cons, Nil};

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new(first_element: Option<u32>) -> List {
        let list = match first_element {
            None => Nil,
            Some(u) => Cons(u, Box::new(Nil)),
        };
        list
    }

    fn prepend(self, new_element: u32) -> List {
        Cons(new_element, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::ch332_enums::List;

    #[test]
    fn new_list() {
        let list = List::new(None);
        assert_eq!(0, list.len());
    }

    #[test]
    fn list_len() {
        let mut list = List::new(None);
        list = list.prepend(10);
        assert_eq!(1, list.len());
    }
}
