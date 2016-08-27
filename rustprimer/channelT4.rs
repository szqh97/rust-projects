use std::sync::mpsc;
use std::thread;
pub fn main() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
    let new_thread = thread::spawn(move || {
        let mut i = 0; 
        loop {
            i = i +1;
            println!("send {}", i);
            match tx.send(i) {
                Ok(_) => (),
                Err(e) => {
                    println!("Send eror: {}, count: {}", e, i);
                    return;
                },
            }
        }
    });
    new_thread.join().unwrap();
    println!("receive: {}", rx.recv().unwrap());
            
}
