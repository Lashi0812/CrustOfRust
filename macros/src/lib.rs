#![allow(dead_code, unused_macros)]

#[macro_export]
macro_rules! avec {

    ($($element:expr),*) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::count!(@COUNT;$($element),*));
        $(vs.push($element);)*
        vs
    }};

    ($($element:expr,)*) =>{{
        $crate::avec![$($element),*]
    }};

    ($element:expr;$count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count,$element);
        vs
    }}

}

#[macro_export]
macro_rules! count {
    (@COUNT;$($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST;$element]),*])
    };

    (@SUBST;$_element:expr) => {
        ()
    };
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert_eq!(x.is_empty(), true);
}

#[test]
fn single() {
    let x: Vec<u32> = avec![1];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 1);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![1, 2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42;2];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
