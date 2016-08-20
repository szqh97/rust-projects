use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,

        }
    }

    fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
        a + b
    }
}

fn main() {
    /*
    println!("{}", add(100i32, 1i32));
    println!("{}", add(100.11f32, 1.1f32));
    */

    let p1 = Point{x: 1, y: 1};
    let p2 = Point{x: 2, y: 2};
    println!("{}", add(p1, p2));
}
