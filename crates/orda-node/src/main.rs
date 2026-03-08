pub mod api;
pub mod execution_engine;
pub mod gas;
pub mod mempool;
pub mod p2p;
pub mod state;

use crate::p2p::{GOSSIPSUB_TOPIC, OrdaBehaviourEvent, create_swarm};
use crate::state::State;
use api::{AppState, create_router};
use colored::*;
use execution_engine::ExecutionEngine;
use libp2p::futures::StreamExt;
use libp2p::gossipsub;
use libp2p::mdns;
use libp2p::swarm::SwarmEvent;
use mempool::TransactionPool;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        "=== Orda Node (Post-Quantum API Gateway) ===".bold().cyan()
    );

    // Initialize Global State
    let state = Arc::new(Mutex::new(State::new()));
    let mempool = Arc::new(Mutex::new(TransactionPool::new()));

    // Spawn the Execution Engine on a background thread
    let execution_mempool = mempool.clone();
    let execution_state = state.clone();
    tokio::spawn(async move {
        ExecutionEngine::run_loop(execution_mempool, execution_state).await;
    });

    // P2P Network Initialization
    let mut swarm = create_swarm().expect("Failed to initialize P2P Swarm");
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .unwrap();

    // Async Channel: API Gateway -> P2P Broadcaster
    let (p2p_tx, mut p2p_rx) = mpsc::unbounded_channel::<String>();

    // Spawn the P2P Networking loop
    let p2p_mempool = mempool.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // 1. Handle outgoing API intents to broadcast
                Some(intent_json) = p2p_rx.recv() => {
                    let topic = gossipsub::IdentTopic::new(GOSSIPSUB_TOPIC);
                    if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, intent_json.as_bytes()) {
                        println!("⚠️ [P2P] Broadcast failed: {:?}", e);
                    } else {
                        println!("🌐 [P2P] Broadcasted Intent to Network");
                    }
                }

                // 2. Handle incoming P2P Swarm events
                event = swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(OrdaBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("🔗 [P2P] Discovered Orda Node: {}", peer_id);
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(OrdaBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            println!("❌ [P2P] Node Expired: {}", peer_id);
                            swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(OrdaBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => {
                        println!(
                            "📥 [P2P] Received Gossip Intent from {peer_id} (msg: {id})"
                        );
                        if let Ok(json_str) = String::from_utf8(message.data) {
                            let mut pool = p2p_mempool.lock().unwrap();
                            // Reuse existing robust mathematical validation logic
                            if let Err(e) = pool.process_incoming_intent(&json_str) {
                                println!("⚠️ [P2P] Rejected remote intent: {}", e);
                            } else {
                                println!("✅ [P2P] Remote Intent cryptographically verified and added to mempool.");
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    // Package the state for the Router
    let app_state = AppState {
        mempool,
        state,
        p2p_sender: p2p_tx,
    };

    let app = create_router(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!(
        "{} TCP Listener Started. Awaiting P2P Intents on {}...",
        "»".yellow(),
        addr.bold().green()
    );

    axum::serve(listener, app).await.unwrap();
}
