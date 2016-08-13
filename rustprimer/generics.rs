enum Option<T> {
    Some(T),
    None,
}

fn make_pair<T,U>(a: T, b: U) -> (T, U) {
    (a, b)
}

struct Point<T> {
    x: T,
    y: T,
}


fn main() {
    let x: Option<i32> = Some(4);
    let y: Option<f64> = Some(5.0f64);
    
    let couple = make_pair("man", "female");

    let in_origin = Point{ x: 0, y: 0 };
    let float_origin = Point {x: 0.0, y:0.0};
}
