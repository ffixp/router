use std::net::Ipv6Addr;

/// Router info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Router {
    /// The router address used for peering
    pub peering_address: Ipv6Addr,
}