fn main() {
    let v0: Vec<i32> = Vec::new();
    let v1: Vec<i32> = vec![];
    let v2 = vec![1,2,3,4,4];
    let v3 = vec![0;10];
    println!("{:?}", v2);

    let hello = "hello world";
    let h2: &'static str = "Hello again";
}
