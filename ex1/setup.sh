#!/bin/sh

BASEDIR="./data"
LARGEFILE="${BASEDIR}/large-input.txt"

echo "creating large file full of random characters..."
openssl rand -out $LARGEFILE -base64 50000000
