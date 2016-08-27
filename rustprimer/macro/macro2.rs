fn foo(x: i32) -> i32 {
    x * x
}

macro_rules! foo {
    ($x: ident) => (
        println!("{:?}", $x);
    )
}

macro_rules! myvec {
    ($($x:expr), *) => (
        {
            let mut tmp_vec = Vec::new();
            $(tmp_vec.push($x);)*
            tmp_vec
        }
    )
}

macro_rules! find_min {
    ($x: expr) => ($x);
    ($x: expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

pub fn main() {
    let a = 5;
    foo!(a);
    println!("{}", foo(a));

    let a = myvec![1,2,3,4];
    println!("{:?}", a);

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32+2, 2u32));
    println!("{}", find_min!(5u32, 2u32*3, 4u32));
}
