use std::fs;
use day_03::{BitCounter, vec_to_usize};

fn main() {
    let file_string = fs::read_to_string("input.txt").unwrap();
    let lines = file_string.lines();

    let mut counter = BitCounter::new(12);
    for line in lines {
        counter.add_bits(line);
    }

    let gamma = counter.get_most_common();
    let epsilon = counter.get_least_common();

    print!("{}\n", vec_to_usize(gamma)*vec_to_usize(epsilon))
}
