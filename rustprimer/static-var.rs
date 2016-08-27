use std::thread;
static mut VAR: i32 = 0;
pub fn main() {
    let new_th = thread::spawn(move || {
        unsafe{
            println!("static value in new thread: {}", VAR);
            VAR = VAR + 1;

        }
    });

    new_th.join().unwrap();
    unsafe {
        println!("static value in main thread: {}", VAR);

    }
}
