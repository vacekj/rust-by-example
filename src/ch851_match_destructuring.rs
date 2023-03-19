// # tuples

fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z_) => println!("named destructuring"),
        (1, ..) => println!("Print first, discard rest"),
        (.., 2) => println!("print last, discard up to the last"),
        (3, .., 4) => println!("print first and last, discard rest"),
        _ => println!("dgaf"),
    }
}

// # arrays/slices

fn array() {
    let array = [3, -2, 6];

    match array {
        [3, second, tail @ ..] => println!(
            "first, second, other elements were:\
        {:?}",
            tail
        ),
        _ => (),
    }
}
