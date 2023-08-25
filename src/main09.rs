use std::{collections::VecDeque, fmt::Display, cmp::min};

struct Game(VecDeque<usize>);

impl Game {
    fn new() -> Game {
        Game(VecDeque::from([0; 1]))
    }

    fn turn(&mut self, marble: usize) -> usize {
        if marble % 23 != 0 {
            // Normal case
            self.0.rotate_left(1);
            self.0.push_back(marble);
            0
        } else {
            self.0.rotate_right(7);
            let take = self.0.pop_back().unwrap();
            self.0.rotate_left(1);
            // println!("Also taking {}", take);
            marble + take
        }
    }

}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current = *self.0.back().unwrap();
        for v in &self.0 {
            if current == *v {
                write!(f, "({}) ", v)
            }  else {
                write!(f, " {}  ", v)
            }?;
        }
        Ok(())
    }
}

fn main() {
    const players: usize = 429;
    const last_marble: usize = 70901*100;

    let mut scores = [0; players as usize];

    let mut game = Game::new();
    for marble in 0..last_marble {
        let player = marble % players;
        scores[player] += game.turn(marble + 1);
        // println!("[{}]\t{}", player + 1, game);
    }

    println!("High score: {}", scores.iter().max().unwrap());
}
