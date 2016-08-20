use std::time;

fn push_1m(v: &mut Vec<usize>, total: usize) {
    let e = time::SystemTime::now();
    for i in 1..total {
        v.push(i);
    }
    let ed = time::SystemTime::now();
    println!("time spend: {:?}", ed.duration_since(e).unwrap());
}

pub fn main() {
    let mut v: Vec<usize> = vec![];
    push_1m(&mut v, 5_000_000);
    let mut v: Vec<usize> = vec![];
    v.reserve(5_000_000);
    push_1m(&mut v, 5_000_000);
}
