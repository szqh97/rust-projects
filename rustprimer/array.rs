fn show(arr: [u8;3]) {
    for i in &arr {
        println!("{}", i);
    }
    
}
fn main() {
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1,2], &array[1..]);

    for x in &array {
        println!("{}", x);
    }

    let a = [8, 9, 10];
    let b: [u8; 3] = [8, 6, 5];
    println!("{}", a[0]);
    println!("............");

    show(a);
}
