# channels

## tl;dr

This section is about how Rust uses channels to communicate information over threads. These channels are multi-producer single consumer channels, so multiple threads can send information over one channel to one thread that reads from the channel. Furthermore, Rust's ownership system is used in these channels to help prevent the sending channels from accidently using the data they send. 

## the details

Here is a basic example of Rust's threads: 

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

The channel consists of two ends: the transmitter and receiver. mpsc stands from multiple-producer single-consumer, so this channel would allow multiple threads to send input into the transmitter side. The transmitter side has a send() method that allows a value to be put into the channel, while the receiver end has a recv() method that takes data out of the channel. recv() is a blocking method, however this is also a try_recv() method that checks immediately, but if it does not find a value it returns an error. 

Something to note in this example is that through the use of 'move' when the thread is created, the ownership of tx is given to that thread. Furthermore, once a value has been sent into the channel, it can now only be used by the receiver. 

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

This piece of code will not run, as it attempts to print val after it is sent through the channel. In the execution, when the code would get to the print statement, the variable val no longer has ownership of the string. This causes as error, as there could be unexpected behavior as the thread on the receiving end of the channel could have modified that data. 

Now we will look at having multiple producer threads with one consumer:

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration; 

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

In this example, two different threads are spawned and given access to the transmitting side of the channel. One of them is given a clone of the transmitting side in order to satisify ownership principles. In this way, it is relatively simple to pass information between threads. 

The code was taken from https://doc.rust-lang.org/book/ch16-02-message-passing.html. 
