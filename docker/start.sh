#!/bin/bash
# This is the entrypoint for the router container
set -e

echo "Starting router!"

# Bring up all Wireguard interfaces
echo "Bringing up all Wireguard interfaces"
/scripts/wg-quick-all.sh up

# Start the webserver
echo "Starting webserver"
mkdir -p /var/www/connect_ffixp_net
caddy start --config /etc/caddy/Caddyfile

# Override the crontab and start crond
echo "Starting crond"
crontab /etc/crontab
crond

# Start the bird daemon
echo "Starting bird daemon"
/usr/sbin/bird -c /etc/bird/bird.conf -d

