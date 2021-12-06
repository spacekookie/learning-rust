use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

mod pool;
use pool::WorkPool;

mod consumer;
use consumer::MessageConsumer;

// Copy your previous solution over!
// mod mailbox;

/// A payload that this application expects
#[derive(Serialize, Deserialize)]
struct Payload {
    msg: String,
    sev: Option<u8>,
}

fn handle_connection(s: &mut TcpStream, pool: &mut WorkPool) -> io::Result<()> {
    let mut buf = vec![];
    s.read_to_end(&mut buf)?;
    pool.queue(buf);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut pool = WorkPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:6000")?;

    // For every incoming connection we first read its message inputs,
    // then forward them to a parser
    for stream in listener.incoming() {
        handle_connection(&mut stream?, &mut pool);
    }

    Ok(())
}
