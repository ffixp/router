#!/bin/bash
# This script will perform a `wg-quick` command on all interfaces listed in /etc/wireguard/*
set -e

for i in /etc/wireguard/*; do
    echo "Processing $i"
    wg-quick "$@" "$i"
done
