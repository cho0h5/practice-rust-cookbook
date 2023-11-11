extern crate crossbeam;
extern crate crossbeam_channel;

use std::thread;
use std::time;
use std::time::Duration;
use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;

fn main() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(1000));
            }
        });
    }).unwrap();
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
