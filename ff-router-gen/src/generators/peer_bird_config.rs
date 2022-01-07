//! This file handles generation of the wireguard config files for each peer

use std::net::Ipv6Addr;

use crate::model::peer::Peer;

use super::ConfigGenerator;

/// Generator for peer bird configs
pub struct PeerBirdConfigGenerator {
    peer: Peer,
}

impl PeerBirdConfigGenerator {
    /// Construct a new PeerBirdConfigGenerator
    pub fn new(peer: Peer) -> Self {
        Self { peer }
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
                    neighbor {} % '{}' as {};
                }}
            "},
            self.peer.asn,
            self.peer.link_local,
            format!("peer{}", self.peer.asn),
            self.peer.asn
        )
    }
}
