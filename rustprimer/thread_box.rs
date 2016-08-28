use std::thread;
use std::sync::Arc;
pub fn main() {
    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();
    let new_th = thread::spawn(move || {
        println!("share value in new thread: {}, address{:p}", share_var, &*share_var);
    });
    new_th.join().unwrap();
    println!("share value in main thread:{}, address:{:p}", var, &*var);
}
