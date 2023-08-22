use std::str::FromStr;

use aoc2018::input_lines;

type Mask = [u8; 26];
fn parse_box_id(s: &String) -> Vec<u32> {
    s.chars().map(|c| c as u32 - 'a' as u32).collect()
}

fn test(a: &[u32], b: &[u32]) -> bool {
    a.iter()
        .zip(b)
        .filter_map(|(x, y)| if x != y {Some(())} else {None})
        .count()
        == 1
}

fn id_as_string(id: &[u32]) -> String {
    let a = 'a' as u32;
    id.iter()
        .map(|c| char::from_u32(c + a).unwrap())
        .collect::<String>()
}

fn main() {
    let input: Vec<Vec<u32>> = input_lines::<String>().iter().map(parse_box_id).collect();

    let mut mask: Mask;

    let mut twos = 0;
    let mut threes = 0;
    let mut is_two;
    let mut is_three;

    // Part 1
    for b in &input {
        mask = [0; 26];
        is_two = false;
        is_three = false;

        for c in b {
            mask[*c as usize] += 1;
        }

        for letter in mask {
            match letter {
                2 => is_two = true,
                3 => is_three = true,
                _ => {}
            }
        }
        if is_two {
            twos += 1;
        }
        if is_three {
            threes += 1;
        }
    }
    println!("Part 1: {}", twos * threes);

    // Part 2
    let mut count = 0;
    for (skip, left) in input.iter().enumerate() {
        for right in input.iter().skip(skip) {
            count += 1;
            if test(left, right) {
                let common_part = left.iter().zip(right).filter_map(|(l,r)| if l == r { Some (*l) } else {None}).collect::<Vec<u32>>();
                println!("Part 2: {}", id_as_string(&common_part));
            }
        }
    }
    println!("Compared {} pairs", count);
}
