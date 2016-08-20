use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T,Output=T>> {
    x: T,
    y: T,
    
}

impl<T: Add<T,Output=T>> Add  for Point<T> {
    type Output=Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    fn add<T: Add<T,Output=T>>(a:T, b:T) -> T {
        a + b
    }
}

fn main () {
    let p1 = Point{x: 1i32, y: 1i32};
    let p2 = Point{x: 1i32, y: 1i32};
    println!("{:?}", add(p1, p2));

    
}
