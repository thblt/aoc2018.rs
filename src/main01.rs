use aoc2018::input_lines;
use std::collections::HashSet;

fn main() {

    let mut acc: HashSet<i64> = HashSet::new();
    let mut freq = 0;
    let input = input_lines::<i64>();

    println!("Part 1: {}", input.iter().sum::<i64>());

    for v in input.into_iter().cycle() {
        freq += v;
        if acc.contains(&freq) {
            println!("Part 2: {}", freq);
            break;
        }  else {
            acc.insert(freq);
        }
    }

}
