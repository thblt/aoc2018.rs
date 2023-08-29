use std::ops::{Add, AddAssign};

use aoc2018::input_lines;

struct<T> Point(T,T);

type MyType<T: Add> = (T,T);


#[derive(PartialEq, Eq)]
enum Track {
    /// No track here!
    None,
    /// The `|` symbol.
    Vertical,
    /// The `-` symbol.
    Horizontal,
    /// The `/` symbol. Name stands for Up Right-Down Left.
    URDL,
    /// The `\` symbol. Name stands for Up Left-Down Right.
    ULDR,
    /// The `+` symbol.
    Intersection,
}

impl From<char> for Track {
    fn from(value: char) -> Self {
        use Track::*;
        match value {
            ' ' => None,
            '|' => Vertical,
            'v' => Vertical,
            '^' => Vertical,
            '-' => Horizontal,
            '>' => Horizontal,
            '<' => Horizontal,
            '/' => URDL,
            '\\' => ULDR,
            '+' => Intersection,
            _ => panic!("I hate your guts."),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CartDirection {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl TryFrom<char> for CartDirection {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CartDirection::*;
        match value {
            '^' => Ok(Up),
            'v' => Ok(Down),
            '<' => Ok(Left),
            '>' => Ok(Right),
            _ => Err(()),
        }
    }
}

impl AddAssign for CartDirection {
    fn add_assign(&mut self, rhs: Self) {
        use CartDirection::*;
        *self = match (*self as isize + rhs as isize) % 4 {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => unreachable!(),
        }
    }
}

impl Add for CartDirection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut ret = self;
        ret += rhs;
        ret
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct Cart {
    /// Location of the cart (an index in the track system)
    location: (isize, isize),
    /// Direction of the cart.
    dir: CartDirection,
    /// The number of times that cart has turned, to calculate its next.
    next_turn: CartDirection,
    removed: bool,
}

impl Cart {
    fn new(location: (isize, isize), dir: CartDirection) -> Self {
        Self {
            location,
            dir,
            next_turn: CartDirection::Left,
            removed: false,
        }
    }

    fn tick(&mut self, ts: &TrackSystem) {
        use CartDirection::*;
        use Track::*;
        // Move
        let mut location = self.location as (isize, isize);
        self.location = match self.dir {
            Up => (location.0, location.1 - 1),
            Right => (location.0 + 1, location.1),
            Down => (location.0, location.1 + 1),
            Left => (location.0 - 1, location.1),
        };

        // Update direction
        self.dir = match ts[self.location.1 as usize][self.location.0 as usize] {
            Vertical => self.dir,
            Horizontal => self.dir,
            URDL => match self.dir {
                Up => Right,
                Right => Up,
                Down => Left,
                Left => Down,
            },
            ULDR => match self.dir {
                Up => Left,
                Right => Down,
                Down => Right,
                Left => Up,
            },
            Intersection => {
                let turn = self.next_turn;
                self.next_turn = match self.next_turn {
                    Up => Right,
                    Left => Up,
                    Right => Left,
                    Down => unreachable!(),
                };
                self.dir + turn
            }
            None => panic!("No track here!"),
        };
    }
}

type TrackLine = Vec<Track>;
type TrackSystem = Vec<TrackLine>;

fn print_system(ts: &TrackSystem, carts: &[Cart]) {
    for (y, line) in ts.into_iter().enumerate() {
        for (x, element) in line.into_iter().enumerate() {
            let cart = carts
                .iter()
                .find(|c| c.location == (x as isize, y as isize));
            if let Some(cart) = cart {
                print!(
                    "{}",
                    match cart.dir {
                        CartDirection::Up => '^',
                        CartDirection::Right => '>',
                        CartDirection::Down => 'v',
                        CartDirection::Left => '<',
                    }
                )
            } else {
                print!(
                    "{}",
                    match element {
                        Track::None => ' ',
                        Track::Vertical => '|',
                        Track::Horizontal => '-',
                        Track::ULDR => '\\',
                        Track::URDL => '/',
                        Track::Intersection => '+',
                    }
                );
            }
        }

        println!();
    }
}

fn main() {
    // Read system
    let input = input_lines::<String>();
    let mut system: TrackSystem = vec![];
    let mut carts = vec![];
    for (y, line) in input.iter().enumerate() {
        let mut system_line = vec![];
        for (x, object) in line.chars().enumerate() {
            let track = Track::from(object);
            system_line.push(track);
            if let Ok(direction) = CartDirection::try_from(object) {
                carts.push(Cart::new((x as isize, y as isize), direction));
            }
        }
        system.push(system_line);
    }

    println!("Lessgo");
    let mut first = true;
    loop {
        carts.sort_by_key(|c| (c.location.1, c.location.0));
        if carts.iter().filter(|c| !c.removed).count() == 1 {
            println!(
                "Part 2: last cart is at {:?}",
                carts.iter().find(|c| !c.removed).unwrap().location
            );
            break;
        }

        for i in 0..carts.len() {
            if carts[i].removed {
                continue;
            }
            carts[i].tick(&system);

            let x = carts[i].location.0;
            let y = carts[i].location.1;
            if carts
                .iter()
                .filter(|c| !c.removed && c.location == (x, y))
                .count()
                > 1
            {
                println!(
                    "{} collision at {},{}",
                    if first { "Part 1:" } else { "(Also) " },
                    x,
                    y
                );
                first = false;
                carts
                    .iter_mut()
                    .filter(|c| c.location == (x, y))
                    .for_each(|c| c.removed = true);
            }
        }
    }
}
