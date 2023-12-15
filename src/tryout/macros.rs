/* Random scribbles from watching Jon's Declarative Macros video https://youtu.be/q6paRBbLgNw?si=9t0ciVBzOMU0e5C3 */

fn new_vec() {}

macro_rules! vek {
    ($elem:literal, $ty:ty) => {
        Vec::with_capacity($elem) as Vec<$ty>;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vek() {
        let a = vek![1, u64];
    }
}
