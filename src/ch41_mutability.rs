fn main() {
    // variables are immutable by default
    let immutable = 42;
    // oh oh, cannot do this
    // immutable = 0;

    let mut able = 0f64;
    able += 1.0; // increment works in Rust, who knew
}
