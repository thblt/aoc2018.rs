const SN: i128 = 3031;

#[inline]
fn power_level(x: i128, y: i128, sn: i128) -> i128 {
    let rack = x + 10;
    let mut power = rack * y;
    power += sn;
    power *= rack;
    power /= 100;
    power %= 10;
    power -= 5;
    power
}

fn test() {
    let mut sum: [[usize; 301]; 301] = [[0; 301]; 301];

    for y in 1..=20 {
        for x in 1..=20 {

            sum[y][x] = 1
                + sum[y - 1][x]
                + sum[y][x - 1]
                - sum[y - 1][x - 1];
        }
    }

    for y in 1..=20 {
        for x in 1..=20 {
            print!("{:3} ", sum[y][x]);
            assert!(sum[y][x] == x*y);
        }
        println!();
    }
}

fn part2() {
    // This is ported from c++ code by Reddit user tribulu at
    // https://www.reddit.com/r/adventofcode/comments/a53r6i/2018_day_11_solutions/ebjogd7/

    // Explaining this to myself: what we're doing here is a partial
    // sum table, or summed area table.  Instead of storing the power
    // level of each cell, each (x,y) cell holds the *total power
    // level* of the area ranging from (1,1) to (x,y).
    //
    // See also https://en.wikipedia.org/wiki/Summed-area_table
    let mut sum: [[i128; 301]; 301] = [[0; 301]; 301];

    // Creating the table.
    for y in 1..=300 {
        for x in 1..=300 {
            sum[y][x] = power_level(x as i128, y as i128, SN)
                + sum[y - 1][x]
                + sum[y][x - 1]
            // Substract here to avoid counting twice!
                - sum[y - 1][x - 1];
        }
    }

    let mut bx: usize = usize::MIN;
    let mut by: usize = usize::MIN;
    let mut bs: usize = usize::MIN;
    let mut best: i128 = i128::MIN;
    let mut bx3: usize = usize::MIN;
    let mut by3: usize = usize::MIN;
    let mut best3: i128 = i128::MIN;
    for s in 1..=300 {
        for y in s..=300 {
            for x in s..=300 {
                let total = sum[y][x] - sum[y - s][x] - sum[y][x - s] + sum[y - s][x - s];
                if total > best {
                    best = total;
                    bx = x;
                    by = y;
                    bs = s;
                }
                if s == 3 && total > best3 {
                    best3 = total;
                    bx3 = x;
                    by3 = y;
                }
            }
        }
    }
    println!(
        "Part 1: Best is ({},{}) with {} power",
        bx3 - 2,
        by3 - 2,
        best3
    );
    println!(
        "Part 2: Best is ({},{},{}) with {} power",
        bx - bs + 1,
        by - bs + 1,
        bs,
        best
    );
}

fn part1() {
    // This is my code
    let mut best = i128::MIN;
    let mut best_coords = (0, 0);
    for x in 1..=298 {
        for y in 1..=298 {
            let mut power = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    power += power_level(x + dx, y + dy, SN);
                }
            }
            if power > best {
                best = power;
                best_coords = (x, y);
            }
        }
    }
    println!(
        "Part 1 (naive version): Best is at ({},{}) with {} power.",
        best_coords.0, best_coords.1, best
    );
}

fn main() {
    part1();
    part2();
    test();
}
