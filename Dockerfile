FROM alpine:latest

# Install everything needed to bring up wireguard
RUN apk update
RUN apk add wireguard-tools
RUN apk add iptables
RUN apk add ip6tables

# Install bird
RUN apk add bird

# Install caddy
RUN apk add caddy

# Install Python3
RUN apk add python3

# Copy in the start script
COPY ./docker/start.sh /start.sh
RUN chmod +x /start.sh

# Start command
CMD ["/start.sh"]