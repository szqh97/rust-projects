#[allow(dead_code)]
fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never turns!");
}


fn main() {
    let num = 5;
    let plus_num = |x: i32| x + num;
    println!("plus_num is {}", plus_num);
}
