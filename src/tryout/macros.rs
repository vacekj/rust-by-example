/* Random scribbles from watching Jon's Declarative Macros video https://youtu.be/q6paRBbLgNw?si=9t0ciVBzOMU0e5C3 */

fn new_vec() {}

macro_rules! vek {
    () => {
        Vec::new()
    };
    ( $($element:expr),+ $(,)* ) => {{
        let mut vs = Vec::new();
        $(vs.push($element);)*
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

    #[test]
    fn double() {
        let x: Vec<u32> = vek![1, 2, 3];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 3);
        assert_eq!(x[0], 1);
        assert_eq!(x[1], 2);
        assert_eq!(x[2], 3);
    }

    #[test]
    fn trailing() {
        let x: Vec<u32> = vek![1, 2, 3, 4,];
        assert_eq!(x.len(), 4);
    }
}
