use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let {sender, receiver} = mpsc::channel();

    let rx = Arc::new(Mutex::new(receiver));

    for i in 0..5 {
        let rx = Arc::clone(&rx);

        thread::spawn(move || {
            let rx = rx.lock().unwrap();
            println!("thread {} receive {}". i, rx.recv().unwrap())
        })
    }

    for i in 0..5 {
        sender.send(i).unwrap();
    }
}