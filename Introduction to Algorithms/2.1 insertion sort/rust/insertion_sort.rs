use std::cmp::{Eq, Ord};
use std::env;
use std::iter::Iterator;
use std::vec::Vec;

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

fn main() {
    let mut sequence = unsorted_vector();
    // the list's size won't change past this point, so optimize memory by freeing reserved
    // capacity in the vector
    sequence.shrink_to_fit();
    insertion_sort(&mut(*sequence));
    let printable_sequence = sequence.iter().map(|&number| number.to_string()).collect::<Vec<String>>();
    println!("{}", printable_sequence.connect(" "));
}

#[test]
fn test_empty_sequence() {
    let mut sequence :Vec<i32> = vec![];
    let expected_output :Vec<i32> = vec![];

    insertion_sort(&mut(*sequence));
    assert!(sequence == expected_output);
}
#[test]
fn test_one_element_sequence() {
    let mut sequence = vec![5];
    let expected_output = vec![5];

    insertion_sort(&mut(*sequence));
    assert!(sequence == expected_output);
}
#[test]
fn test_many_element_sequence() {
    let mut sequence = vec![-8, 923, 17, 1, 15, -72, -23849, 0, 129];
    let expected_output = vec![-23849, -72, -8, 0, 1, 15, 17, 129, 923];

    insertion_sort(&mut(*sequence));
    assert!(sequence == expected_output);
}
