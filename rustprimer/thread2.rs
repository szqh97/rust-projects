use std::thread;

pub fn main() {
    let new_thread = thread::spawn(move || {
        println!("I am a new thread...");
    });
    new_thread.join().unwrap();

    let new_thread_result = thread::Builder::new()
                            .name("thread1".to_string())
                            .stack_size(4*1024*1024).spawn(
                                move||{
                                    loop {
                                        
                                        println!("I am a thread");
                                    }
                            });
    new_thread_result.unwrap().join().unwrap();
}
