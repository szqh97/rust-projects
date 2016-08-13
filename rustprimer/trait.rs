use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T,K>(x: T, y: K)
    where T: Clone,
          K: Clone + Debug
{
    x.clone();
    y.clone();
    println!("{:?}", y);
}





trait Foo {
    fn foo(&self);
    fn bar(&self) { println!("We called bar.");}
}

trait FooBar: Foo {
    fn foobar(&self);
}

struct Baz;

impl Foo for Baz {
    fn foo(&self) { println!("foo"); }
}

impl FooBar for Baz {
    fn foobar(&self) { println!("foobar"); }
    fn bar(&self) { println!("bar in Baz"); }
}

fn main() {
    let c = Circle{x: 0.0, y: 0.0, radius: 10.0};
    let s = Square{x: 1.0, y:1.0, side:10.0};
    print_area(c);
    print_area(s);


    foo(1,2);
    bar(3, 4);

    let b = Baz{};
    b.foo();
    b.foobar();
    b.bar();

}
