# FFIXP Router
[![Build](https://github.com/ffixp/router/actions/workflows/build.yml/badge.svg)](https://github.com/ffixp/router/actions/workflows/build.yml)
[![Clippy](https://github.com/ffixp/router/actions/workflows/clippy.yml/badge.svg)](https://github.com/ffixp/router/actions/workflows/clippy.yml)
[![Audit](https://github.com/ffixp/router/actions/workflows/audit.yml/badge.svg)](https://github.com/ffixp/router/actions/workflows/audit.yml)

This repository contains everything required to bootstrap the FFIXP router service. The process is as follows:

- JSON configs are parsed
- Wireguard configs are generated for each peer
- Bird2 configs are generated for each peer
- Additional scripts and cron jobs are set up
- Docker container is bootstrapped
- `docker-compose` handles the rest of the work of building and setting up the router

The goal of this project is to make it super easy to add new peers to FFIXP. IX operators just have to update `configs/peers.json` and re-run the bootstrap program.

## The configs

Due to sensitive information, some configs are gitignored and are not included in the repository.

`configs/router.json` example:

```json
{
    "asn": 4242420966,
    "peering_address": "fd70:4904:a771::1",
    "peering_link_local_address": "fe80::a771",
    "routed_ipv4": "xx.xx.xx.xx",
    "routed_ipv6": "xxxx:xx:xx:xx::x",
    "public_gateway": "example.com:1234",
    "public_gateway_pubkey": "asdfasdfasdfasdfasdfasdsdf"
}
```

`configs/wg_privkey` simply contains the priavte key of the router.

## Running the router

```sh
# Generate everything
cargo run -- generate --peer-file configs/peers.json --private-key configs/wg_privkey --router-file configs/router.json

# Build and run the container
docker-compose build
docker-compose up -d
```