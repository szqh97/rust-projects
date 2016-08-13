fn main() {
    let f = true;
    let f: bool = false;

    let c = 'c';
    let x = 43;

    let y: u32 = 123_456;
    let z: f64 = 1.23e+2;
    let zero = z.abs_sub(123.4);
    let bin = 0b111_000;
    let oct = 0o7326_1545;
    let hex = 0xf23a_b049;

    let str = "Hello, world";
    let mut string = str.to_string();

    let a = [0, 1, 2, 3, 4, 5];
    let middle = &a[1..4];
    let mut ten_zeros: [i64; 10] = [0; 10];

    //tuples
    let tuple: (i32, &str) = (50, "hello");
    let (fifty, _) = tuple;
    let hello = tuple.1;

    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe{ *raw };

    println!("{:?}", points_at);

    //function
    fn foo(x: i32) -> i32 {x}
    let bar: fn(i32) -> i32 = foo;

    let decimal = 65.344_132;
    let integer = decimal as u8;
    println!("decimal is {:?}, integer is {:?}", decimal, integer);
}
