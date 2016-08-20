fn main() {
    /*
    let x: i32 = 100;
    let s = move|i : i32| i +x;
    let y = s(2);
    println!("{}, {}", x, y);
    */

    let mut x: String = String::from("abc");
    let mut s = move |c: char| x.push(c);
    let y = s('d');
    //println!("{:?}", x);
}

