use std::cmp::{Eq, Ord};
use std::env;
use std::iter::Iterator;
use std::vec::Vec;

fn main() {
    let mut sequence = unsorted_vector();
    sequence.shrink_to_fit();
    //println!("Unsorted: {:?}", &sequence);
    insertion_sort(&mut(*sequence));
    //println!("Sorted: {:?}", &sequence);
}

fn unsorted_vector() -> Vec<i32> {
    env::args().skip(1).map(|num_string|
        num_string.parse::<i32>().ok().expect("Couldn't parse input string to an integer.")
    ).collect()
}

fn insertion_sort<T: Eq + Ord>(sequence: &mut [T]) {
    for j in 1..sequence.len() + 1 {
        let mut i = j - 1;
        while i > 0 && sequence[i - 1] > sequence[i] {
            sequence.swap(i, i - 1);
            i = i - 1;
        }
    }
}
