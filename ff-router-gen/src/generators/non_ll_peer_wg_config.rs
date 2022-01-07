//! This file handles generation of the wireguard config files for each peer

use crate::model::{peer::Peer, router::Router};

use super::ConfigGenerator;

/// Generator for peer wg configs
pub struct NonLLPeerWireguardConfigGenerator {
    peers: Vec<Peer>,
    wg_privkey: String,
    router: Router,
}

impl NonLLPeerWireguardConfigGenerator {
    /// Construct a new NonLLPeerWireguardConfigGenerator
    pub fn new(peers: Vec<Peer>, wg_privkey: &str, router: &Router) -> Self {
        Self {
            peers,
            wg_privkey: wg_privkey.to_string(),
            router: router.clone(),
        }
    }
}

impl ConfigGenerator for NonLLPeerWireguardConfigGenerator {
    /// The config file name
    fn filename(&self) -> String {
        "mesh.conf".to_string()
    }

    /// The config file contents
    fn generate(&self) -> String {
        format!(
            indoc::indoc! {"
                [Interface]
                PrivateKey = {}
                Address = {}/48

                {}
            "},
            self.wg_privkey,
            self.router.peering_address,
            self.peers.iter().map(|peer| {
                format!(
                    indoc::indoc! {"
                    [Peer]
                    PublicKey = {}
                    AllowedIPs = 172.16.0.0/12, 10.0.0.0/8, fd00::/8, fe80::/10
                    PersistentKeepalive = 25
                    Endpoint = {}
                    "},
                    peer.pubkey,
                    peer.endpoint.as_ref().expect("Non link-local peers must have endpoints")
                )
            }).collect::<Vec<String>>().join("\n")
        )
    }
}
