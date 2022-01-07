pub mod peer_wg_config;
pub mod gateway_wg_config;
pub mod peer_bird_config;
pub mod global_bird_config;
pub mod non_ll_peer_wg_config;

/// Generic trait for a configuration generator
pub trait ConfigGenerator {
    /// The config file name
    fn filename(&self) -> String;
    /// The config file contents
    fn generate(&self) -> String;
}