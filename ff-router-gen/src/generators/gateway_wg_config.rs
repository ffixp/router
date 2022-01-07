//! This file handles generation of the wireguard config file for the gateway

use crate::model::router::Router;

use super::ConfigGenerator;

/// Generator for the public gateway wg config
pub struct PublicGatewayWireguardConfigGenerator {
    router: Router,
    wg_privkey: String,
}

impl PublicGatewayWireguardConfigGenerator {
    /// Construct a new PublicGatewayWireguardConfigGenerator
    pub fn new(router: Router, wg_privkey: &str) -> Self {
        Self {
            router,
            wg_privkey: wg_privkey.to_string(),
        }
    }
}

impl ConfigGenerator for PublicGatewayWireguardConfigGenerator {
    /// The config file name
    fn filename(&self) -> String {
        "gateway.conf".to_string()
    }

    /// The config file contents
    fn generate(&self) -> String {
        format!(
            indoc::indoc! {"
                [Interface]
                PrivateKey = {}
                Address = {}/32, {}/128
                DNS = 1.1.1.1, 2606:4700:4700::1111, 8.8.8.8, 2001:4860:4860::8888

                [Peer]
                PublicKey = {}
                Endpoint = {}
                AllowedIPs = 0.0.0.0/0,::/0
                PersistentKeepalive = 25
            "},
            self.wg_privkey,
            self.router.routed_ipv4,
            self.router.routed_ipv6,
            self.router.public_gateway_pubkey,
            self.router.public_gateway,
        )
    }
}
