/* Random scribbles from watching Jon's Declarative Macros video https://youtu.be/q6paRBbLgNw?si=9t0ciVBzOMU0e5C3 */

fn new_vec() {}

macro_rules! vek {
    () => {
        Vec::new()
    };
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vek() {
        let a: Vec<u32> = vek!();
        assert!(a.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = vek![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }
}
