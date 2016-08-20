use std::collections::HashMap;


pub fn main() {
    let mut letters = HashMap::new();
    for ch in "a short treatesies on fungi".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", letters);
}
