#!/bin/sh

BASEDIR="./data"
EX111DATA="${BASEDIR}/ex1-1-1-large-input.txt"
EX113DATA="${BASEDIR}/ex1-1-3-large-input-with-empty-line.txt"
EX116DATA="${BASEDIR}/ex1-1-6-large-input-with-random-length-of-lines.txt"

echo "creating dataset for ex1-1-1, full of random characters..."
openssl rand -out $EX111DATA -base64 50000000

echo "creating dataset for ex1-1-3, full of random characters containing empty line..."
openssl rand -out $EX113DATA -base64 50000000
sed -i '51G' $EX113DATA

echo "creating dataset for ex1-1-6, full of random characters with random length of lines..."
openssl rand -base64 10000000 > $EX116DATA
openssl rand -base64 10000000 | awk '{printf $1"\n"$1"\n"}' >> $EX116DATA
sed -i -e 's/a//2g' $EX116DATA
