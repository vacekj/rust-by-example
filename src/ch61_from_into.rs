use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    /* From trait allows for a type to define how to create itself
    from another type */
    let my_str = "hello";
    let my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let into_num: Number = int.into();
    println!("My number is {:?}", num);
}
