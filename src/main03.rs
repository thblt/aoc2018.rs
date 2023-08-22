use std::str::FromStr;
use std::collections::HashMap;

use aoc2018::input_lines;
use sscanf::sscanf;

#[derive(Eq,PartialEq,Ord,PartialOrd,Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = sscanf!(s, "#{u32} @ {u32},{u32}: {u32}x{u32}")?;
        Ok(Claim {
            id: parts.0,
            x: parts.1,
            y: parts.2,
            width: parts.3,
            height: parts.4,
        })
    }
}

fn main() {
    let claims = input_lines::<Claim>();
    let mut squares: HashMap<(u32, u32), u32> = HashMap::new();

    for claim in &claims {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                *squares.entry((x,y)).or_insert(0) += 1;
            }
        }
   }

    println!("Part 1: {}", squares.values().filter(|v| v>&&1).count());

    'main: for claim in &claims {
        let mut winner = false;
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if squares[&(x,y)] > 1 {
                    continue 'main
                }
            }
        }
        println!("Part 2: {}", claim.id);
        break
    }
}
