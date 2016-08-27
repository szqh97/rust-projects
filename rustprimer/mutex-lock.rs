use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;


pub fn main() {

    const N: usize = 10;
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move ||{
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
        });
    }

    rx.recv().unwrap();
    println!("{:?}", data);

}
