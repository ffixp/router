use std::net::{Ipv6Addr, Ipv4Addr};

/// Router info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    /// The router ASN
    pub asn: u32,
    /// The router address used for peering
    pub peering_address: Ipv6Addr,
    /// The public IPv4 address of the router as provided by gateway
    pub routed_ipv4: Ipv4Addr,
    /// The public IPv6 address of the router as provided by gateway
    pub routed_ipv6: Ipv6Addr,
    /// The gateway endpoint
    pub public_gateway: String,
    /// The gateway public key
    pub public_gateway_pubkey: String,
}