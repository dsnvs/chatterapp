use futures::{prelude::*};

use libp2p::{
    floodsub::{self, FloodsubEvent},
    identity, mdns,
    swarm::{SwarmEvent},
    mplex, tcp,
    noise,
    core::upgrade,
    Transport, PeerId, Swarm,
};
use std::{error::Error, hash::Hash, collections::HashMap};
use tokio::io::{self, AsyncBufReadExt};

use crate::p2p::{MyBehaviour,};
use crate::logic::{Lamport, MessageType, LamportMessage};
use crate::logic::Message as CustomMessage;

mod logic;
mod p2p;
mod generic;

enum ConnectionStatus {
    CONNECTED,
    DISCONNECTED
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let mut peer_map = HashMap::new();


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

    let lamport = floodsub::Topic::new("comms");

    swarm.behaviour_mut().floodsub.subscribe(lamport.clone());

    // Read full lines from stdin
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Listen on all interfaces and a port the assigned by the OS
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Start event loop
    loop {
        tokio::select! {
            line = stdin.next_line() => {
                let line = line?.expect("stdin closed");
                let message_data = bincode::serialize("Test message").unwrap();
                let mut args = line.split(' ');
                let test_message = match args.next() {
                    Some("A") => {
                        CustomMessage::new(MessageType::LamportMessage(LamportMessage::GenericTransaction),
                            Some(local_peer_id),
                            local_peer_id,
                            message_data,
                        )
                    },
                    (_) => {
                        CustomMessage::new(MessageType::LamportMessage(LamportMessage::GenericTransaction),
                        None,
                        local_peer_id,
                        message_data,
                        )
                    }
                };
                
                swarm.behaviour_mut().floodsub.publish(lamport.clone(), test_message.as_bytes());
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening in {address:?}");
                },
                SwarmEvent::Behaviour(p2p::MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, multiaddr) in list {
                        println!("discovered {peer_id} {multiaddr}");
                        swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer_id);
                        peer_map.insert(peer_id, ConnectionStatus::CONNECTED);
                    }
                },
                SwarmEvent::Behaviour(p2p::MyBehaviourEvent::Floodsub(FloodsubEvent::Message(message))) => {
                    println!("Received message from {:?}", message.source);
                    handle_overlay_protocol(local_peer_id, &message.data);
                }
                _ => {}
            }
        }
    }
}


fn handle_overlay_protocol(peer_id: PeerId, serialized_message: &[u8]) {
    let message: CustomMessage = bincode::deserialize(serialized_message).unwrap();

    if message.to.is_some() && message.to.unwrap() != peer_id {
        println!("I am not the recipient");
        return
    }

    match message.message_type {
        MessageType::LamportMessage(message_type) => match message_type {
            LamportMessage::GenericTransaction => {
                println!("Handler TESTRPC")
            }
            LamportMessage::GenericTransaction2 => todo!(),
        },
        MessageType::MutexMessage(_) => todo!(),
        MessageType::DistributedConsensusMessage(_) => todo!(),
    }
}

fn input_handling(local_peer_id: PeerId, line: String) -> CustomMessage {
    let message_data = bincode::serialize("Test message").unwrap();
    let mut args = line.split(' ');
    
    let test_message = match args.next() {
        Some("A") => {
            return CustomMessage::new(MessageType::LamportMessage(LamportMessage::GenericTransaction),
                Some(local_peer_id),
                local_peer_id,
                message_data,
            )
        },
        (_) => {
            return CustomMessage::new(MessageType::LamportMessage(LamportMessage::GenericTransaction),
            None,
            local_peer_id,
            message_data,
            )
        }
    };
}