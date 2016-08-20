fn main() {
    let a = [1,2,3,4,5,6,7];
    let mut b = Vec::<i32>::new();
    for i in &a {
        b.push(get_func(*i)(*i));
    }
    println!("{:?}", b);
}

fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }

    fn dec(n: i32) -> i32 {
        n - 1
    }

    if n % 2 == 0 {
        inc
    } else {
        dec
    }
    
}
