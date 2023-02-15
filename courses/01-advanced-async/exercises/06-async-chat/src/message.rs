//! This is the only part of the system that is pre-specified
//!
//! You can change this message structure of course, but it will give
//! you some guidance on how to proceed.

pub enum Message {
    /// A text message sent from one person to a room, or relayed from
    /// a room to a client
    TextPayload {
        room: String,
        nick: String,
        content: String,
    },
    /// A request to join a room.  You _can_ add another message type
    /// for leaving rooms, but this can usually also be handled via
    /// the connection state
    JoinRoom { room: String, nick: String },
    /// A request to create a room
    CreateRoom { room: String },
    // Potentially missing: ShutdownRequest and/or ShutdownNotice
}
