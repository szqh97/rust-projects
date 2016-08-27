use std::sync::Arc;
use std::thread;

pub fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let child_numbers = shared_numbers.clone();
        thread::spawn(move ||{
            let mut local_numbers = &child_numbers[..];
            let mut i = local_numbers.get(3).unwrap();
            println!("{:?}", i);
        });
    }
}
