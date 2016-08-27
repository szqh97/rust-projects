use std::sync::mpsc;
use std::thread;

pub fn main() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("reveive {}", rx.recv().unwrap());
}
