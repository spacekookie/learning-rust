use async_chat::*;

fn main() {
    // 1. Initialise some kind of runtime
    // 2. Listen for connections on a socket (TCP, UDP, Unix, ... again, up to you)
    // 3. Listen for user input (for example via stdin)
    // 4. For each "line" of input...
    //     - Parse input into command/ Message
    //     - Send message to server
    //     - Respond to server reply

    // How do you handle shutdowns?  Can the client shut down the
    // server?  Or does the client only respond to shutdowns?
}
