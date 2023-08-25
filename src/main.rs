use std::mem::transmute;

use aoc2018::*;

fn read_node(raw: &[u32], mut index: usize, meta_mode: bool) -> (u32, usize) {
    let children_count = raw[index];
    let meta_count = raw[index+1];
    index += 2;

    let children_mode = !meta_mode && children_count > 0;
    let mut child_values = vec!();
    let mut result = 0;

    for _ in 0..children_count {
        let x;
        (x, index) = read_node(&raw, index, meta_mode);
        child_values.push(x);
        if meta_mode { // For part 1
            result += x;
        }
    }

    for _ in 0..meta_count {
        let meta = raw[index];
        index += 1;
        if children_mode {
            let meta = meta as usize;
            if meta <= child_values.len() {
                result += child_values[meta - 1];
            }
        } else {
            result += meta;
        }
    }
    (result, index)
}

fn main() {
    let data = input_lines::<String>()[0]
        .split(" ")
        .map(|x| str::parse::<u32>(x).unwrap())
        .collect::<Vec<u32>>();
    let (result1, _) = read_node(&data, 0, true);
    let (result2, _) = read_node(&data, 0, false);
    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
