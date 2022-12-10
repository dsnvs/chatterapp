use libp2p::floodsub::{Floodsub, FloodsubEvent};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{Kademlia, KademliaEvent};
use libp2p::request_response::{RequestResponse, RequestResponseEvent, ProtocolSupport, RequestResponseConfig};
use libp2p::{
    identity, mdns,
    swarm::{NetworkBehaviour},
    PeerId,
};
use std::time::Duration;

use crate::generic::{GenericCodec, IncomingRequest, OutgoingResponse, GenericProtocol};


pub type GenericRequestResponseEvent = RequestResponseEvent<IncomingRequest, OutgoingResponse>;

// We create a custom network behaviour that combines Kademlia and mDNS.
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
pub struct MyBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub generic_request_response: RequestResponse<GenericCodec>,
    pub floodsub: Floodsub,
}

#[allow(clippy::large_enum_variant)]
pub enum MyBehaviourEvent {
    Kademlia(KademliaEvent),
    Mdns(mdns::Event),
    RequestResponse(GenericRequestResponseEvent),
    Floodsub(FloodsubEvent)
}


impl From<KademliaEvent> for MyBehaviourEvent {
    fn from(event: KademliaEvent) -> Self {
        Self::Kademlia(event)
    }
}

impl From<mdns::Event> for MyBehaviourEvent {
    fn from(event: mdns::Event) -> Self {
        Self::Mdns(event)
    }
}

impl From<GenericRequestResponseEvent> for MyBehaviourEvent {
    fn from(event: GenericRequestResponseEvent) -> Self {
        Self::RequestResponse(event)
    }
}

impl From<FloodsubEvent> for MyBehaviourEvent {
    fn from(event: FloodsubEvent) -> Self {
        Self::Floodsub(event)
    }
}

impl MyBehaviour {
    pub fn new(_local_key: identity::Keypair, local_peer_id: PeerId) -> Self {

        // Kademlia behaviour
        let store = MemoryStore::new(local_peer_id);
        let kademlia = Kademlia::new(local_peer_id, store);

        // Mdns behaviour
        let mdns = mdns::tokio::Behaviour::new(mdns::Config {
            ttl: Duration::from_secs(6 * 60),
            query_interval: Duration::from_secs(10),
            enable_ipv6: false
        }).unwrap();

        // Generic Request Response behaviour
        let generic_request_response = RequestResponse::new(
            GenericCodec,
            vec![(GenericProtocol, ProtocolSupport::Full)],
            RequestResponseConfig::default(),
        );

        // Floodsub behaviour
        let floodsub = Floodsub::new(local_peer_id);

        Self {
            kademlia,
            mdns,
            generic_request_response,
            floodsub
        }
    }

}