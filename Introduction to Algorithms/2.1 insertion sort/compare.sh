#!/usr/bin/env bash

n=1000000

input="$RANDOM"
for i in $(seq 1 1 $(($n-1)))
do
	input="$input $RANDOM"
done

format='elapsed time: %E  CPU usage: %P  Mem usage: %MKb'

rust_release="rust/insertion_sort_release $input"
rust_debug="rust/insertion_sort_debug $input"
ruby="ruby/insertion_sort.rb $input"

echo "Time to sort $n elements:"
echo Rust relase build:
/usr/bin/time -f "$format" -- $rust_release 1>/dev/null
echo Rust debug build:
/usr/bin/time -f "$format" -- $rust_debug 1>/dev/null
echo Ruby:
/usr/bin/time -f "$format" -- $ruby 1>/dev/null
