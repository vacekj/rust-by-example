use std::fmt;
use std::fmt::Formatter;

struct Circle {
    radius: u32;
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // convert any type to string by implementing the ToString trait
    // or, better yet, a fmt::Display trait
    let circle = Circle { radius: 10 };
    println!("{}", circle.to_string());
}
