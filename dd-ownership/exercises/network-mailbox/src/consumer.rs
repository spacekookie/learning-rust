use std::thread;
use std::time::{sleep, Duration};

pub struct MessageConsumer {
    dur: Duration,
    // TODO: add reference to your mailbox here
}

impl MessageConsumer {
    pub fn start(dur: Duration) {
        Self { dur }.run();
    }

    fn run(self) {
        thread::spawn(move || {
            // Get _all_ new messages as a chunk
            // while let Some(msg) = self.mailbox.pop() {
            //     println!("Message received: {}", msg);
            // }

            // Then sleep for a while
            sleep(self.dur.clone()).unwrap();
        })
    }
}
