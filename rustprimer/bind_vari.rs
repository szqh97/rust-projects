fn main() {
    /*
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2);

    let b1:u32= 5;
    //assert_eq!(a1, b1);
    //
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    a = 2.0;
    println!("{:?}", a);

    */

    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b={:?}", a, b);

    b = true;
    assert_eq!(a, b);
    
}
