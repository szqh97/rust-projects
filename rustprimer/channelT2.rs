use std::fmt;
use std::sync::mpsc;
use std::thread;
use std::rc::Rc;
use std::marker::Send;

pub struct Student {
    id: u32
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Student {}", self.id)
    }
}

impl !Send for Student {
}
pub fn main() {
    let (tx, rx): (mpsc::Sender<Rc<Student>>, mpsc::Receiver<Rc<Student>>) = mpsc::channel();
    thread::spawn(move || {
        tx.send(Rc::new(Student{
            id: 1,
        })).unwrap();
    });

    println!("reveive {}", rx.recv().unwrap());
}
