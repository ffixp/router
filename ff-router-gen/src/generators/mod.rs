pub mod peer_wg_config;

/// Generic trait for a configuration generator
pub trait ConfigGenerator {
    /// The config file name
    fn filename(&self) -> String;
    /// The config file contents
    fn generate(&self) -> String;
}