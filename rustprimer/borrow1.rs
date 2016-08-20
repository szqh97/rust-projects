fn main() {
    let mut x: Vec<i32> = vec!(1i32, 2, 3);
    {
        let y = &mut x;
        y.push(100);
        println!("{:?}", y);
    }
    println!("{:?}", x);
}
