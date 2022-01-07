#!/bin/bash
# This is the entrypoint for the router container
set -e

echo "Starting router!"

# Set up the address space we announce for testing
# echo "Setting up test address space on dummy0"
# ip link add dummy0 type dummy
# ip -6 address add fd70:4904:a771::1/48 dev dummy0

# Bring up all Wireguard interfaces
echo "Bringing up all Wireguard interfaces"
/scripts/wg-quick-all.sh up

# Start the bird daemon
echo "Starting bird daemon"
/usr/sbin/bird -c /etc/bird/bird.conf -d

