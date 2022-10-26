pub fn main() {
    // variable bindings have a scope, and are constrained to live in a block
    // also, variable shadowing is (unfortunately) allowed

    let shadowed_binding = 1;
    {
        println!("Var before being shadowed: {}", shadowed_binding);

        /* This binding shadows the previous one */
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);
}
