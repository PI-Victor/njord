
/// EventSink channels events for a specific action.
pub struct EventSink {

}

pub enum NodeEvent {
    /// Init is an event that is sent when the node wants to initialize as part
    /// Raft.
    Init,
    MessageReceive,
    Accepted,
    Validated,
    Incoming
}

pub enum ClientEvent {
    MessageReceive,
    Accepted,
    Validated,
}
