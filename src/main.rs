use std::ops::Deref;
use aoc2018::*;

// Meta mode is for part 1: just accumulate metadata.
fn read_node<T>(raw: &mut T, meta_mode: bool) -> u32
where
    T: Iterator,
 <T as Iterator>::Item: Deref<Target = u32>
{
    let children_count = raw.next().unwrap();
    let meta_count = raw.next().unwrap();

    let children_mode = !meta_mode && *children_count > 0;
    let mut child_values = vec![];
    let mut result = 0;

    for _ in 0..*children_count {
        if meta_mode {
            result += read_node(raw, meta_mode);
        } else {
            child_values.push(read_node(raw, meta_mode));
        }
    }

    for _ in 0..*meta_count {
        let meta = raw.next().unwrap();
        if children_mode {
            let meta = *meta as usize;
            if meta <= child_values.len() {
                result += child_values[meta - 1];
            }
        } else {
            result += *meta;
        }
    }
    result
}

fn main() {
    let data = input_lines::<String>()[0]
        .split(" ")
        .map(|x| str::parse::<u32>(x).unwrap())
        .collect::<Vec<u32>>();
    println!("Part 1: {}", read_node(&mut data.iter(), true));
    println!("Part 2: {}", read_node(&mut data.iter(), false));
}
