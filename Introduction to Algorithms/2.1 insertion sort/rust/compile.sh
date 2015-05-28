#!/usr/bin/env bash

rustc -C opt-level=3 -o insertion_sort_release insertion_sort.rs
rustc -g -o insertion_sort_debug insertion_sort.rs
