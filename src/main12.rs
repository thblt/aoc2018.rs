use aoc2018::input_lines;

// I did part 2 by hand, so there's no clean code here.  The trick is
// that at some point, the plant pattern remains identical and simply
// shifts to the right, one step at each generation.  Given we've
// found this pattern at generation g , with s the sum of plant
// positions (like the result of part 1), pots containing plants, and
// n the number of plant-containing pots, then the solution to part
// two is:
//
// s + ((50000000000-g)*n)
//
// Simply put: the current solution, plus the number of remaining
// generations multiplied by the number of plants (right shifting by 1
// means the solution increases by 1 for each plant)

const POTS: usize = 10000;
const SHIFT: isize = 5000;

// Notes
// Since a note is just a five-bits number, we encode it as a number.
type Plants = Vec<bool>;
type Notes = [bool; 2 << 4];

fn main() {
    let (mut plants, notes) = read_input();

    print!(" 0: ");
    print_plants(&plants);
    for x in 1..=200 {
        step(&mut plants, &notes);
        print!("{:2}: ", x);
        print_plants(&plants);
    }

    let extra_gens: u128 = 50000000000-200;
    // let extra_gens: u128 = 200-100;
    let result = 16866 + extra_gens*80;
    // let result = 8866 + extra_gens*80;
    println!("{}", result)

}

fn print_plants(plants: &Plants) {
    let mut buffer: String = String::new();
    let mut started = false;
    let mut first = true;
    let mut count = 0;
    let mut result = 0;
    for (n, plant) in plants.iter().enumerate() {
        started |= plant;
        if !started {
            continue;
        } else {
            if first {
                print!(" {} ", n as isize - SHIFT);
                first = false;
            }

            if *plant {
                result += n as isize - SHIFT;
                count += 1;
                print!("{}#", buffer);
                buffer = String::new();
            } else {
                buffer += ".";
            }
        }

    }
    println!(" ({} pots, pos total: {})", count, result);
}

fn step(plants: &mut Plants, notes: &Notes) {
    let old = plants.clone();
    for p in 2..old.len() - 3 {
        let rule = read_number(&old[p - 2..p + 3]);
        plants[p] = notes[rule];
    }
}

fn read_input() -> (Plants, Notes) {
    let mut plants = [false; POTS].to_vec();
    let mut notes: Notes = [false; 2 << 4];
    let input = input_lines::<String>();
    input[0]
        .split_once(": ")
        .unwrap()
        .1
        .chars()
        .map(|c| c.as_bool())
        .enumerate()
        .for_each(|(n, p)| plants[n + (SHIFT as usize)] = p);
    for (cond, effect) in input.iter().skip(2).map(|s| s.split_once(" => ").unwrap()) {
        notes[read_number(&cond.chars().collect::<Vec<char>>())] =
            effect.chars().nth(0).unwrap().as_bool();
    }
    (plants, notes)
}

fn read_number<T: Boolish>(n: &[T]) -> usize {
    // println!("Reading {} bits", n.len());
    let mut x = 1;
    let mut ret = 0;
    for d in n {
        if d.as_bool() {
            ret += x;
        }
        x *= 2;
    }
    ret
}

trait Boolish {
    fn as_bool(self: &Self) -> bool;
}

impl Boolish for char {
    fn as_bool(self: &Self) -> bool {
        match self {
            '#' => true,
            '.' => false,
            _ => panic!("Bad char {}", self),
        }
    }
}

impl Boolish for bool {
    fn as_bool(self: &Self) -> bool {
        *self
    }
}
