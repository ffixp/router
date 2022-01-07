//! This file handles generation of the wireguard config files for each peer

use crate::model::{peer::Peer, router::Router};

use super::ConfigGenerator;

/// Generator for peer wg configs
pub struct LinkLocalPeerWireguardConfigGenerator {
    peer: Peer,
    wg_privkey: String,
    router: Router,
}

impl LinkLocalPeerWireguardConfigGenerator {
    /// Construct a new LinkLocalPeerWireguardConfigGenerator
    pub fn new(peer: Peer, wg_privkey: &str, router: &Router) -> Self {
        Self {
            peer,
            wg_privkey: wg_privkey.to_string(),
            router: router.clone(),
        }
    }
}

impl ConfigGenerator for LinkLocalPeerWireguardConfigGenerator {
    /// The config file name
    fn filename(&self) -> String {
        format!("peer{}.conf", self.peer.asn)
    }

    /// The config file contents
    fn generate(&self) -> String {
        format!(
            indoc::indoc! {"
                [Interface]
                PrivateKey = {}
                PostUp = ip addr add {}/128 peer {}/128 dev %i
                Table = off
                {}

                [Peer]
                PublicKey = {}
                AllowedIPs = 172.16.0.0/12, 10.0.0.0/8, fd00::/8, fe80::/10
                PersistentKeepalive = 25
                {}
            "},
            self.wg_privkey,
            self.router.peering_link_local_address,
            self.peer.ipv6,
            match &self.peer.listen_port {
                Some(port) => format!("ListenPort = {}", port),
                None => String::new(),
            },
            self.peer.pubkey,
            match &self.peer.endpoint {
                Some(wg_endpoint) => format!("Endpoint = {}", wg_endpoint),
                None => String::new(),
            }
        )
    }
}
