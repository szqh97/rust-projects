use std::sync::{Arc, Mutex, Condvar};
use std::thread;

pub fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("notify main thread");
    }).join().unwrap();

    let &(ref lock , ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before waited");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}
