use crate::Payload;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

/// A round-robin worker pool
pub struct WorkPool {
    ring: VecDeque<Sender<Vec<u8>>>,
}

impl WorkPool {
    // TODO: provide the WorkPool with a reference to your mailbox
    pub fn new(num: u8) -> Self {
        // basically this is just a loop via the
        // Iterator API.  You could just as easily have written "for _ in
        // (0..num)" but that would have been too simple :)
        let ring = (0..num)
            .map(|_| {
                let (tx, rx) = channel();
                thread::spawn(move || Worker::new(rx).run());
                tx
            })
            .collect();

        Self { ring }
    }

    pub fn queue(&mut self, buf: Vec<u8>) {
        // TODO: implement round-robin work queuing
    }
}

struct Worker {
    rx: Receiver<Vec<u8>>,
    // TODO: add a reference to your mailbox here
}

impl Worker {
    fn new(rx: Receiver<Vec<u8>>) -> Self {
        Self { rx }
    }

    fn run(self) {
        while let Ok(vec) = self.rx.recv() {
            match serde_json::from_slice::<Payload>(&vec) {
                Ok(msg) => { /* TODO: queue payload in mailbox */ }
                Err(e) => {
                    eprintln!("Received invalid payload: '{}'!", e);
                }
            }
        }
    }
}
