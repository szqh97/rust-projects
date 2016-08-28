use std::thread;
use std::sync::{Arc, Mutex};
pub fn main() {
    let var: Arc<Mutex<u32>> = Arc::new(Mutex::new(5));
    let share_var = var.clone();

    let new_th = thread::spawn(move || {
        let mut val = share_var.lock().unwrap();
        println!("shared value in new thread:{}", *val);
        *val = 9;
    });

    new_th.join().unwrap();
    println!("share value in main thread : {}", *(var.lock().unwrap()));
}
