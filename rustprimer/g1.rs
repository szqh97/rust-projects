use std::ops::Add;

fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(100i32, 1i32));
    println!("{}", add(100.11f32, 1.11f32));
}
