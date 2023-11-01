#!/bin/env bash
openssl req -x509 -nodes -extensions v3_req -days 3650 -newkey rsa:4096 \
    -subj "/C=US/ST=California/L=San Francisco/O=ACME Inc/CN=prometheus" \
    -keyout /etc/ssl/private/server.key -out /etc/ssl/certs/server.crt
