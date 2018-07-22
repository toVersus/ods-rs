#!/bin/sh

BASEDIR="./data"
EX111DATA="${BASEDIR}/ex1-1-1-large-input.txt"

echo "creating dataset for ex1-1-1, full of random characters..."
openssl rand -out $EX111DATA -base64 50000000
