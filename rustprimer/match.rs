fn main() {
    let day = 4;
    match day {
        0 | 6  => println!("Weekend"),
        e@1...5 => println!("Weekday, {:?}", e),
        _ => println!("invalid"),
    }

    let x = 5;
    let mut y = 5;
    match x {
        ref r => println!("Got a referene to {}", r),
    }

    match y {
        ref mut mr  => println!("Got a mutable ref {}", mr),
    }

    let pair = (0, -3);
    match pair {
        (0, v) => println!("x is 0 and y is {}", v),
        (v, 0) => println!("x is {} and y is 0", v),
        _ => println!("It dosn't matter with they are"),
    }

    /////////////
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point{x: 0, y: 0};
    match origin {
         Point{x, .. } =>  println!("x is {}", x),
    }

    enum OptionalInt {
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than 5"),
        OptionalInt::Value(..)  => println!("Go an Int"),
        OptionalInt::Missing => println!("no such luck"),
    }

    let number = Some(7);
    let mut optional = Some(6);

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }
    else {
        println!("Didn't match a number");
    }

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit");
            optional = None;
        } else {
            println!("i is {:?}, Try again", i);
            optional = Some(i + 1);
        }
    }

}
