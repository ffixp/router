//! The code backing the generate command

use clap::ArgMatches;

use crate::{
    generators::{
        gateway_wg_config::PublicGatewayWireguardConfigGenerator,
        peer_bird_config::PeerBirdConfigGenerator, peer_wg_config::PeerWireguardConfigGenerator,
        ConfigGenerator, global_bird_config::GlobalBirdConfigGenerator,
    },
    model::{peer::Peer, router::Router},
};

fn mkdir(dir: &str) {
    let output_dir = std::path::Path::new(dir);
    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir).unwrap();
    }
}

/// Performs the actual generation step
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
    mkdir("./generated/bird/peers");

    // Handle generating all peer wireguard and birdconfigs
    for peer in peers {
        // Wireguard
        {
            // Generate the config
            let generator = PeerWireguardConfigGenerator::new(
                peer.clone(),
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

        // Bird
        {
            // Generate the config
            let generator = PeerBirdConfigGenerator::new(peer);
            let filename = generator.filename();
            let contents = generator.generate();

            // Write the config to disk
            std::fs::write(format!("./generated/bird/peers/{}", filename), contents).unwrap();
        }
    }

    // Handle generating the gateway wireguard config
    {
        let gateway_generator =
            PublicGatewayWireguardConfigGenerator::new(router_config.clone(), &wg_private_key);
        let filename = gateway_generator.filename();
        let contents = gateway_generator.generate();

        // Write the config to disk
        std::fs::write(
            format!("./generated/wireguard/interfaces/{}", filename),
            contents,
        )
        .unwrap();
    }

    // Handle generating the router bird config
    {
        let generator = GlobalBirdConfigGenerator::new(router_config);
        let filename = generator.filename();
        let contents = generator.generate();

        // Write the config to disk
        std::fs::write(format!("./generated/bird/{}", filename), contents).unwrap();
    }

    // Copy the static bird files
    std::fs::copy("./bird/peer_template.conf", "./generated/bird/peer_template.conf").unwrap();
    std::fs::copy("./bird/router.conf", "./generated/bird/router.conf").unwrap();
}
