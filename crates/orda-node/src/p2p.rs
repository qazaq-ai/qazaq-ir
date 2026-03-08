use libp2p::gossipsub::{self, MessageAuthenticity, ValidationMode};
use libp2p::mdns;
use libp2p::swarm::NetworkBehaviour;
use libp2p::{Swarm, SwarmBuilder, identity, noise, tcp, yamux};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::time::Duration;

pub const GOSSIPSUB_TOPIC: &str = "orda-mempool";

/// Custom Network Behaviour combining peer discovery (mDNS) and publish/subscribe (Gossipsub).
#[derive(NetworkBehaviour)]
pub struct OrdaBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

/// Initializes the P2P Swarm for the Orda Node.
pub fn create_swarm() -> Result<Swarm<OrdaBehaviour>, Box<dyn Error>> {
    // 1. Generate a cryptographic identity for the local peer
    let keypair = identity::Keypair::generate_ed25519();
    let local_peer_id = keypair.public().to_peer_id();
    println!("🌍 [P2P] Local Node Identity: {}", local_peer_id);

    // 2. Build the TCP Transport with Noise encryption and Yamux multiplexing
    let mut swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            // Setup Gossipsub for Broadcasting intents
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(5))
                .validation_mode(ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .expect("Valid config");

            let gossipsub = gossipsub::Behaviour::new(
                MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )
            .expect("Valid gossipsub behaviour");

            // Setup mDNS for automatic local peer discovery
            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            Ok(OrdaBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // 3. Subscribe to the Global Mempool Gossip channel
    let topic = gossipsub::IdentTopic::new(GOSSIPSUB_TOPIC);
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    Ok(swarm)
}
