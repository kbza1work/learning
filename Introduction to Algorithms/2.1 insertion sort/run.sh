#!/usr/bin/env bash

n=10000

input=""
for i in $(seq 1 1 $n)
do
	input="$input $RANDOM"
done

rust="./insertion_sort $input"

echo "Time to sort $n elements:"
echo Rust:
rust_time=$(/usr/bin/time -f'%E' $rust)
