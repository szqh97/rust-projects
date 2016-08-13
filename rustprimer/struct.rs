fn main() {
    struct Poin {
        x: i32,
        y: i32,
    }

    let mut point = Poin{x: 0, y: 0};

    struct Color(u8, u8, u8);
    let andriod_green = Color(0xa4, 0xc0, 0x32);
   // let (red, green, blue) = andriod_green;

    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    println!("{}", d);


}
