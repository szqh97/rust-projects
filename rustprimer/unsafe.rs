pub fn main() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe{ *raw };
    println!("raw points at {}", points_at);
    static mut N: i32 = 5;
    unsafe {
        N += 1;
        println!("N: {}", N);
    }
}
