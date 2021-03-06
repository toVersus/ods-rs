#!/bin/bash

set -o braceexpand

BASEDIR="./data"
EX111DATA="${BASEDIR}/ex1-1-1-large-input.txt"
EX113DATA="${BASEDIR}/ex1-1-3-large-input-with-empty-line.txt"
EX116DATA="${BASEDIR}/ex1-1-6-large-input-with-random-length-of-lines.txt"
EX12DATA="${BASEDIR}/ex1-2-dyck-words.txt"
EX13DATA="${BASEDIR}/ex1-3-is-closed-grouping-statement.txt"
EX14DATA="${BASEDIR}/ex1-4-sequence-of-numbers.txt"

echo "creating dataset for ex1-1-1, full of random characters..."
openssl rand -out $EX111DATA -base64 50000000

echo "creating dataset for ex1-1-3, full of random characters containing empty line..."
openssl rand -out $EX113DATA -base64 50000000
sed -i '51G' $EX113DATA

echo "creating dataset for ex1-1-6, full of random characters with random length of lines..."
openssl rand -base64 10000000 > $EX116DATA
openssl rand -base64 10000000 | awk '{printf $1"\n"$1"\n"}' >> $EX116DATA
sed -i -e 's/a//2g' $EX116DATA

echo "creating dyck words for ex1-2"
printf '+1-1%.0s' {1..100} > $EX12DATA

echo "creating a pair of braces, backets and parenthesises"
printf '{[()]}%.0s' {1..100} > $EX13DATA

echo "creating a sequence of numbers for ex1-4"
seq 1 1000 > $EX14DATA
