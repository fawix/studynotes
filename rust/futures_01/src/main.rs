//from: http://asquera.de/blog/2017-03-01/the-future-with-futures/
extern crate futures_01; //this crate
extern crate futures;// rust futures crate

use futures_01::sleep_a_little_bit;
use std::thread;
use futures::Future; 
use futures::sync::oneshot;


fn main() {
    println!("Hello, Oneshot Futures!!");

    // This is a simple future built into the crate which feel sort of like
    // one-time channels. You get a (sender, receiver) when you invoke them.
    // Sending a value consumes that side of the channel, leaving only the reciever.
    let (tx, rx) = oneshot::channel();

    // We can spawn a thread to simulate an action that takes time, like a web
    // request. In this case it's just sleeping for a random time.
    thread::spawn(move || {
        println!("--> START");

        let waited_for = sleep_a_little_bit();
        println!("--- WAITED {}", waited_for);
        // This consumes the sender, we can't use it afterwards.
        tx.send(waited_for);

        println!("<-- END");
    });

    // Now we can wait for it to finish
    let result = rx.wait()
        .unwrap();

    // This value will be the same as the previous "WAITED" output.
    println!("{}", result);

}

