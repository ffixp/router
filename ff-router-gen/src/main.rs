#![feature(drain_filter)]

#[macro_use]
extern crate serde;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand, AppSettings};
use generate::do_config_generation;

mod generate;
mod generators;
mod model;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("generate")
            
                .alias("gen")
                .about("Generates everything required to run the router")
                .arg(
                    Arg::with_name("wg_privkey")
                        .long("private-key")
                        .help("The private key of the wireguard endpoint")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("peers")
                        .long("peer-file")
                        .help("JSON file describing BGP peers")
                        .takes_value(true)
                        .required(true),
                ).arg(
                    Arg::with_name("router_config")
                        .long("router-file")
                        .help("JSON file describing this router")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("generate") {
        println!("Generating configs...");
        do_config_generation(matches);
    } else {
        unimplemented!();
    }
}
