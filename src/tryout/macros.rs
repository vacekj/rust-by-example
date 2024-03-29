/* Random scribbles from watching Jon's Declarative Macros video https://youtu.be/q6paRBbLgNw?si=9t0ciVBzOMU0e5C3 */

fn new_vec() {}

/// A hand-written vec macro for learning purposes
#[macro_export]
macro_rules! vek {
    ($($element:expr),*) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::vek![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};

    ($($element:expr,)*) => {{
       $crate::vek![$($element),*]
    }};

    ( $element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

    /* Counting in macros is weird */
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::vek![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
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

    #[test]
    fn invalid_expr() {
        /* this won't compile, because any compilation error that happens in the expanded macro
        gets propagated to the calling code*/
        // let v: Vec<usize> = vek!(42, "hello");
    }

    /// ```compile_fail
    /// let v: Vec<usize> = vek!(42, "hello");
    /// ```
    struct CompileFailtest;

    #[test]
    fn with_capacity() {
        let v: Vec<usize> = vek!(42, 5);
    }

    /* Video time: 54:54 */
}
