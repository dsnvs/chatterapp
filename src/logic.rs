use libp2p::{PeerId};
use serde::{Serialize, Deserialize};
use core::fmt;
use std::error::Error;
#[derive(Debug)]
struct SyncError {
    details: String
}

// Error that will be raised whenever a transaction has a syncing problem

impl SyncError {
    fn new(msg: &str) -> SyncError {
        SyncError { details: msg.to_string() }
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SyncError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Standard timestamp format
type Timestamp = u64;

// Dummy transaction
#[derive(Clone, Serialize, Deserialize)]
pub struct LamportTransaction {
    content: String,
    timestamp: Timestamp
}

// Lamport timestamp object
#[derive(Clone, Serialize, Deserialize)]
pub struct Lamport {
    timestamp: Timestamp
}

impl Lamport {
    pub fn new() -> Self {
        Self { timestamp: 0 }
    }

    pub fn update(&mut self, transaction_timestamp: Timestamp) {
        self.timestamp = std::cmp::max(self.timestamp, transaction_timestamp);
    }

    pub fn tick(&mut self) {
        self.timestamp += 1;
    }

    pub fn get_timestamp(&self) -> Timestamp {
        self.timestamp
    }
}


/* Create enum for different types of events that can be encoded, then prefix all events with this enum */
/* Create a serializer that takes this enum type as the first argument */
/* Implement different handlers for each value */

/*
    I am creating an abstract protocol that can support different types of messages
    Each Message is a wrapper containing:
    - message_type: an enum of enums that defines which type of call is being made
    - to: an optional peerId value in case this message is meant to a specific node
    - from: the sender's peerId
    - data: the message's data, serialized to a vec<U8> ahead of time


    This abstraction also includes a serialize and deserialize trait

    Message handling should be implemented as:
        Deserialize
        Match Message_Type
            Message_Type<Message_Type> => {
                HANDLING FOR THIS MESSAGE_TYPE
            }
            Message_Type<Message_Type> => {
                HANDLING FOR THIS MESSAGE_TYPE
            }
            Message_Type<Message_Type> => {
                HANDLING FOR THIS MESSAGE_TYPE
            }
            _ => {
                Some sort of error handling.
                Most likely error type is corruption as there are 3 layers of lossy encoding for each message
            }
*/

#[derive(Serialize, Deserialize)]
enum MessageType {
    LamportMessage(LamportMessage),
    MutexMessage(MutexMessage),
    DistributedConsensusMessage(DistributedConsensusMessage)
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    message_type: MessageType,
    to: Option<PeerId>,
    from: PeerId,
    data: String,
}

impl Message {
    fn new(message_type: MessageType, to: Option<PeerId>, from: PeerId, data: String) -> Self {
        Self {
            message_type,
            to,
            from,
            data,
        }
    }
}
#[derive(Serialize, Deserialize)]
enum LamportMessage {
    TESTRPC
}
#[derive(Serialize, Deserialize)]
enum MutexMessage {

}
#[derive(Serialize, Deserialize)]
enum DistributedConsensusMessage {
    
}



/* fn test_function() {
    let message = Message::new(MessageType::LamportMessage(LamportMessage::TESTRPC), None, PeerId::random(), "Test Data".to_string());
} */