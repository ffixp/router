use std::net::Ipv6Addr;

/// A BGP peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Peer name
    pub name: String,
    /// Optional URL for info about the peer
    pub homepage: Option<String>,
    /// Link-local v6 address of the peer
    pub link_local: Ipv6Addr,
    /// ASN
    pub asn: u32,
    /// Wireguard public key
    pub pubkey: String,
    /// Optional Wireguard endpoint
    pub wg_endpoint: Option<String>,
    /// Optional Wireguard endpoint port
    pub listen_port: Option<u16>,
}