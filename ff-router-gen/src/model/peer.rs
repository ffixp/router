use std::net::Ipv6Addr;

/// A BGP peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Peer name
    pub name: String,
    /// Optional URL for info about the peer
    pub homepage: Option<String>,
    /// v6 address of the peer
    pub ipv6: Ipv6Addr,
    /// Weather the peer address is link-local
    pub is_link_local: Option<bool>,
    /// ASN
    pub asn: u32,
    /// Wireguard public key
    pub pubkey: String,
    /// Optional Wireguard endpoint
    pub endpoint: Option<String>,
    /// Optional Wireguard endpoint port
    pub listen_port: Option<u16>,
}

impl Peer {
    /// Generate the interface name for this peer.
    pub fn interface_name(&self) -> String {
        // This is not valid for non-link-local peers
        if !self.is_link_local.unwrap_or(false) {
            panic!("Cannot generate interface name for non-link-local peer");
        }

        format!("peer{}", self.asn)
    }

    pub fn address_formatted(&self) -> String {
        match self.is_link_local.unwrap_or(false) {
            true => format!("{}%{}", self.ipv6, self.interface_name()),
            false => format!("{}", self.ipv6),
        }
    }
}
