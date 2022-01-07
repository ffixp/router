//! The code backing the generate command

use clap::ArgMatches;

use crate::{
    generators::{peer_wg_config::PeerWireguardConfigGenerator, ConfigGenerator, gateway_wg_config::PublicGatewayWireguardConfigGenerator},
    model::{peer::Peer, router::Router},
};

fn mkdir(dir: &str) {
    let output_dir = std::path::Path::new(dir);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }
}

pub fn do_config_generation(matches: &ArgMatches) {
    // Load the configs
    let peers: Vec<Peer> = serde_json::from_str(
        std::fs::read_to_string(matches.value_of("peers").unwrap())
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let wg_private_key: String = std::fs::read_to_string(matches.value_of("wg_privkey").unwrap())
        .unwrap()
        .trim()
        .to_string();
    let router_config: Router = serde_json::from_str(
        std::fs::read_to_string(matches.value_of("router_config").unwrap())
            .unwrap()
            .as_str(),
    )
    .unwrap();

    // Print some statistics
    println!("{} peers", peers.len());

    // Ensure the output directory exists
    mkdir("./generated");
    mkdir("./generated/wireguard/interfaces");

    // Handle generating all peer wireguard configs
    for peer in peers {
        // Generate the config
        let generator = PeerWireguardConfigGenerator::new(
            peer,
            &wg_private_key,
            &router_config.peering_address,
        );
        let filename = generator.filename();
        let contents = generator.generate();

        // Write the config to disk
        std::fs::write(
            format!("./generated/wireguard/interfaces/{}", filename),
            contents,
        )
        .unwrap();
    }

    // Handle generating the gateway wireguard config
    {
        let gateway_generator =
            PublicGatewayWireguardConfigGenerator::new(router_config, &wg_private_key);
        let filename = gateway_generator.filename();
        let contents = gateway_generator.generate();

        // Write the config to disk
        std::fs::write(
            format!("./generated/wireguard/interfaces/{}", filename),
            contents,
        ).unwrap();
    }
}
