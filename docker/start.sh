#!/bin/bash
# This is the entrypoint for the router container
set -e

echo "Starting router!"

# Bring up all Wireguard interfaces
echo "Bringing up all Wireguard interfaces"
/scripts/wg-quick-all.sh up

