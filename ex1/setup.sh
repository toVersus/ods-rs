#!/bin/sh

BASEDIR="./data"
EX111DATA="${BASEDIR}/ex1-1-1-large-input.txt"
EX113DATA="${BASEDIR}/ex1-1-3-large-input-with-empty-line.txt"

echo "creating dataset for ex1-1-1, full of random characters..."
openssl rand -out $EX111DATA -base64 50000000

echo "creating dataset for ex1-1-3, full of random characters containing empty line..."
openssl rand -out $EX113DATA -base64 50000000
sed -i '51G' $EX113DATA
