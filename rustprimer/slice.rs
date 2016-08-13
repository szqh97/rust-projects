fn show(arr: &[u8]) {
    for i in arr {
        println!("{}", i);
    }
    println!("");
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let slice_a = &arr[..];
    let b = &arr[1..5];
    show(slice_a);
    show(b);

    let mut v1: Vec<i32> = vec![1,2,3];
    let v2 = vec![0; 10];
    println!("{}", v1[0]);
    println!("xxxxxxxx");

    for i in &v1 {
        print!("{}", i);
    }
    println!("");

    for i in &mut v1 {
        *i = *i+1;
        println!("{}", i);
    }

}
