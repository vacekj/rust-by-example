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
    ( $element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        let x = $element;
        for _ in 0..$count {
            vs.push(x.clone());
        }
        vs
    }}
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
        let x: Vec<u32> = vek![1, 2, 3, 4,,,,,,,,];
        assert_eq!(x.len(), 4);
    }

    #[test]
    fn invalid_expr() {
        /* this won't compile, because any compilation error that happens in the expanded macro
        gets propagated to the calling code*/
        // let v: Vec<usize> = vek!(42, "hello");
    }

    #[test]
    fn with_capacity() {
        let v: Vec<usize> = vek!(42, 5);
    }
}
