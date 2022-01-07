#! /usr/bin/env python3
# This script will take the output of `birdc show protocol` through stdin, and export it as JSON to stdout.
import sys
import re
import json

# The REGEX expression for parsing data from birdc's peer list
BIRD_PROTO_RE = re.compile(
    r"^([a-zA-Z\d]+)\s+([a-zA-Z\d]+)\s+([a-zA-Z-\d]+)\s+([a-zA-Z]+)\s+([\d:.]+)\s*([a-zA-Z]+)?.*$", flags=re.MULTILINE)

# A list for all peers
peers = []

# Handle the incoming data
line_num = 0
for line in sys.stdin:
    # The first two lines are garbage, so we skip them
    if line_num < 2:
        line_num += 1
        continue

    # We can then just handle each line as a peer
    line_data = BIRD_PROTO_RE.findall(line)
    if line_data:
        line_data = line_data[0]
        peers.append(
            {
                "name": line_data[0],
                "protocol": line_data[1],
                "table": line_data[2],
                "state": line_data[3],
                "since": line_data[4],
                "info": line_data[5] if len(line_data) >= 6 else None
            }
        )

# Dump the peer list to STDOUT
print(json.dumps(peers, indent=4))
