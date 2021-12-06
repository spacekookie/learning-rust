use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Implement this mailbox which can queue and pop messages from its
/// internal state without requiring external mutable access.
///
/// Instead use the Mutex provided within.
#[derive(Clone)]
struct Mailbox {
    inner: Arc<Mutex<VecDeque<String>>>,
}

impl Mailbox {
    /// Create a new mailbox object
    pub fn new() -> Self {
        todo!()
    }
    
    /// Queue a new message to the back
    pub fn queue(&self, s: String) {
        todo!()
    }
    
    /// Pop the first message off the front
    pub fn pop(&self) -> Option<String> {
        todo!()
    }
}

/// Optionally you can use this "Generator" abstraction to spawn
/// threads.  Some code is missing
pub struct Generator {
    mb: crate::Mailbox,
}

impl Generator {
    pub fn start(mb: Mailbox) {
        Self { mb }.spawn();
    }
    
    fn spawn(self) {
        thread::spawn(move || {
            todo!()
        });
    }
}

fn main() {
    let mb = Mailbox::new();
    
    { // Use this thread to generate message (2x)
        let mb = mb.clone();
        thread::spawn(|| { todo!() });
    }
    
    while let Some(msg) = mb.pop() {
        println!("Message received: {}", msg);
    }
    
    println!("No more messages...");
}
