use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

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
        Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Queue a new message to the back
    pub fn queue(&self, s: String) {
        self.inner.lock().unwrap().push_back(s);
    }

    /// Pop the first message off the front
    pub fn pop(&self) -> Option<String> {
        self.inner.lock().unwrap().pop_front()
    }
}

/// Optionally you can use this "Generator" abstraction to spawn
/// threads.  Some code is missing
pub struct Generator {
    mb: crate::Mailbox,
}

impl Generator {
    fn start(mb: Mailbox) {
        Self { mb }.spawn();
    }

    fn spawn(self) {
        thread::spawn(move || {
            let mb = self.mb;
            for i in 0..20 {
                mb.queue(format!("String #{}", i));
            }
        });
    }
}

fn main() {
    let mb = Mailbox::new();

    Generator::start(mb.clone());
    Generator::start(mb.clone());

    std::thread::sleep_ms(100);
    
    while let Some(msg) = mb.pop() {
        println!("Message received: {}", msg);
    }

    println!("No more messages...");
}
