//! This file handles generation of the wireguard config files for each peer

use std::net::Ipv6Addr;

use crate::model::peer::Peer;

use super::ConfigGenerator;

/// Generator for peer wg configs
pub struct PeerWireguardConfigGenerator {
    peer: Peer,
    wg_privkey: String,
    address: Ipv6Addr,
}

impl PeerWireguardConfigGenerator {
    /// Construct a new PeerWireguardConfigGenerator
    pub fn new(peer: Peer, wg_privkey: &str, address: &Ipv6Addr) -> Self {
        Self {
            peer,
            wg_privkey: wg_privkey.to_string(),
            address: address.clone(),
        }
    }
}

impl ConfigGenerator for PeerWireguardConfigGenerator {
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
                PostUp = ip addr add dev %i {}/128 peer {}/128
                Table = off
                {}

                [Peer]
                PublicKey = {}
                AllowedIPs = 172.16.0.0/12, 10.0.0.0/8, fd00::/8, fe80::/10
                {}
            "},
            self.wg_privkey,
            self.address,
            self.peer.link_local,
            match &self.peer.listen_port {
                Some(port) => format!("ListenPort = {}", port),
                None => String::new(),
            },
            self.peer.pubkey,
            match &self.peer.wg_endpoint {
                Some(wg_endpoint) => format!("Endpoint = {}", wg_endpoint),
                None => String::new(),
            }
        )
    }
}
