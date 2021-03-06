//! This file handles generation of the bird config files for each peer

use std::net::Ipv6Addr;

use crate::model::peer::Peer;

use super::ConfigGenerator;

/// Generator for peer bird configs
pub struct PeerBirdConfigGenerator {
    peer: Peer,
}

impl PeerBirdConfigGenerator {
    /// Construct a new PeerBirdConfigGenerator
    pub fn new(peer: &Peer) -> Self {
        Self { peer: peer.clone() }
    }
}

impl ConfigGenerator for PeerBirdConfigGenerator {
    /// The config file name
    fn filename(&self) -> String {
        format!("as{}.conf", self.peer.asn)
    }

    /// The config file contents
    fn generate(&self) -> String {
        format!(
            indoc::indoc! {"
                protocol bgp as{} from peers {{
                    source address {};
                    neighbor {} as {};
                }}
            "},
            self.peer.asn,
            match self.peer.is_link_local.unwrap_or(false) {
                true => "SELF_IPV6_ADDR_LINK_LOCAL",
                false => "SELF_IPV6_ADDR",
            },
            self.peer.address_formatted(),
            self.peer.asn
        )
    }
}
