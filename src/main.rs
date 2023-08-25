use std::{fmt::Display, collections::VecDeque};

struct Game {
    // The board
    board: Vec<usize>,
    // Index of the current marble
    current: isize,
}

impl Game {
    fn new() -> Game {
        Game {
            board: vec![0],
            current: 0,
        }
    }

    fn turn(&mut self, marble: usize) -> usize {
        if marble % 23 != 0 {
            // Normal case
            let position = self.current + 2;
            self.current = self.insert(position, marble);
            0
        } else {
            const SHIFT: isize = -7;
            let take = self.normalize_index(self.current + SHIFT) as isize;
            let score = marble + self.remove(take);

            if take == self.board.len() as isize {
                self.current = 0;
            } else {
                self.current = self.normalize_index(take) as isize;
            }
            score
        }
    }

    fn normalize_index(&self, index: isize) -> usize {
        if index < 0 {
            self.normalize_index(self.board.len() as isize + index)
        } else if index as usize >= self.board.len() {
            index as usize % self.board.len()
        } else {
            index as usize
        }
    }

    fn insert(&mut self, index: isize, marble: usize) -> isize {
        let mut index = self.normalize_index(index);
        if index == 0 {
            index = self.board.len();
        }
        self.board.insert(index, marble);
        index as isize
    }

    fn remove(&mut self, index: isize) -> usize {
        let index = self.normalize_index(index);
        self.board.remove(index)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.iter().find(|x| **x == 0);
        for (pos, marble) in self.board.iter().enumerate() {
            if pos == self.current as usize {
                write!(f, "({}) ", marble)
            } else {
                write!(f, " {}  ", marble)
            }?;
        }
        Ok(())
    }
}

fn main() {
    const players: usize = 429;
    const last_marble: usize = 70901;

    let mut scores = [0; players as usize];

    let mut game = Game::new();
    for marble in 0..last_marble {
        let player = marble % players;
        scores[player] += game.turn(marble + 1);
        // println!("[{}]\t{}", player + 1, game);
    }

    println!("High score: {}", scores.iter().max().unwrap());
}
