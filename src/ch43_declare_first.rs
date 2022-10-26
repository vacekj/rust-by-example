pub fn main() {
    // declare a var binding
    let a_binding;

    {
        let x = 2;

        // init the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of unitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
