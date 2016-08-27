#![feature(box_syntax, box_patterns)]
#![warn(unused_variables)]
use std::rc::Rc;
pub fn main() {
    let boxed = Some(box 5);
    match boxed {
        Some(box unboxed) => println!("Some {}", unboxed),
        None => println!("None"),

    }

    let f = Rc::new(5);
    let f2 = f.clone();
    let f3 = f.clone();
}
