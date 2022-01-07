#!/bin/bash
# This script is to be run in a cron job
# This just grabs, reformats, and saves bird peers
set -e

birdc show protocol | python3 /scripts/bird_peers_to_json.py > /var/www/connect_ffixp_net/peers.json