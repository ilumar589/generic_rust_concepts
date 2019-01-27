use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;

pub fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn thread_takes_ownership_of_variable() {
    let v = vec![1, 2, 3];

    let thread_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    thread_handle.join().unwrap();
}

// message passing between threads
pub fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // ! unwrap destroys the result instance because the function moves the instance. it takes self instead of &self
    }); //this all happens in another thread

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn another_message_passing() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move|| {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Got {}", received);
    }
}

pub fn using_shared_data() {

}