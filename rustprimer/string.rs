fn main() {
    let x = "xxxxf要地在A".to_string();
    for i in x.as_bytes() {
        println!("{}", i);
    }

    println!("");

    for i in x.chars()  {
        println!("{}", i);
    }
}
