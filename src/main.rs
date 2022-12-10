use futures::{prelude::*};
use libp2p::floodsub::{self, FloodsubEvent, Topic};
use libp2p::request_response::{RequestResponseEvent, RequestResponse};
use libp2p::{
    identity, mdns,
    swarm::{SwarmEvent},
    mplex, tcp,
    noise,
    core::upgrade,
};
use libp2p::{
    Transport, PeerId, Swarm,
    request_response::RequestResponseMessage,
};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use std::time::SystemTime;

use crate::p2p::{MyBehaviour,};
use crate::logic::Lamport;

mod logic;
mod p2p;
mod generic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Lamport 
    // let lamport = Lamport::new();

    println!("Local peer id: {:?}", local_peer_id);

    // Create a tokio-based TCP transport use noise for authenticated
    // encryption and Mplex for multiplexing of substreams on a TCP stream.
    let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
    .upgrade(upgrade::Version::V1)
    .authenticate(
        noise::NoiseAuthenticated::xx(&local_key)
            .expect("Signing libp2p-noise static DH keypair failed."),
    )
    .multiplex(mplex::MplexConfig::new())
    .boxed();

    // Create the node NetworkBehaviour
    let behaviour = MyBehaviour::new(local_key, local_peer_id);
    
    // Create a swarm using the transport and behaviour specified previously
    let mut swarm = Swarm::with_tokio_executor(transport, behaviour, local_peer_id);

    let lamport = floodsub::Topic::new("lamport");
    let mutex = floodsub::Topic::new("mutex");

    swarm.behaviour_mut().floodsub.subscribe(lamport.clone());
    swarm.behaviour_mut().floodsub.subscribe(mutex.clone());

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and a port the assigned by the OS
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Start event loop
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                swarm.behaviour_mut().floodsub.publish(lamport.clone(), line.as_bytes());
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                },
                SwarmEvent::Behaviour(p2p::MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        println!("discovered {peer_id} {multiaddr}");
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());
                        swarm.behaviour_mut().generic_request_response.add_address(&peer_id, multiaddr);
                        swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer_id);
                    }
                },
                SwarmEvent::Behaviour(p2p::MyBehaviourEvent::Floodsub(FloodsubEvent::Message(message))) => {
                    println!("{:?} sent '{:?}'", message.source, String::from_utf8_lossy(&message.data));
                }
                SwarmEvent::Behaviour(p2p::MyBehaviourEvent::RequestResponse(RequestResponseEvent::Message {
                    peer: _,
                    message:
                        RequestResponseMessage::Request {
                            request, channel, ..
                        },
                })) => {

                    // TEST CODE
                    let req: String = bincode::deserialize(&request).unwrap();
                    println!("Received: {:?}", req);
                    
                    let response = 5;
                    swarm.behaviour_mut().generic_request_response.send_response(channel, bincode::serialize(&response).unwrap());
                    // TEST CODE
                },
                _ => {}
            }
        }
    }
}
