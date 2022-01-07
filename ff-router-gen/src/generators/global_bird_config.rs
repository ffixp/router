//! This file handles generation of the global bird config

use std::net::Ipv6Addr;

use crate::model::{peer::Peer, router::Router};

use super::ConfigGenerator;

/// Generator for global bird config
pub struct GlobalBirdConfigGenerator {
    router: Router,
}

impl GlobalBirdConfigGenerator {
    /// Construct a new GlobalBirdConfigGenerator
    pub fn new(router: Router) -> Self {
        Self { router }
    }
}

impl ConfigGenerator for GlobalBirdConfigGenerator {
    /// The config file name
    fn filename(&self) -> String {
        "bird.conf".to_string()
    }

    /// The config file contents
    fn generate(&self) -> String {
        format!(
            indoc::indoc! {"
            # Listening settings           
            router id {};

            # Define some vars for use by sub-configs
            define SELF_ASN = {};
            define SELF_IPV6_ADDR = {};

            # Pull in the router config
            include \"/etc/bird/router.conf\";

            # Pull in the peer template
            include \"/etc/bird/peer_template.conf\";

            # Pull in the peer configs
            include \"/etc/bird/peers/*\";
            
            "},
            self.router.routed_ipv4, self.router.asn, self.router.peering_link_local_address
        )
    }
}
