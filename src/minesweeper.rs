use crate::rand::random_range;
use std::fmt::Display;
use std::{
    collections::HashSet,
    fmt::{Write},
};


pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_case: HashSet<Position>,
    mines: HashSet<Position>,
    flagged: HashSet<Position>,
    lost: bool,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if !self.open_case.contains(&pos) {
                    if self.flagged.contains(&pos) {
                        f.write_str(" 🚩 ")?;
                    } else {
                        f.write_str(" 🟪 ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str(" 💣 ")?;
                } else {
                    write!(f, " {} ", self.neighboring_mines(pos))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_case: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged: HashSet::new(),
            lost: false,
        }
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item=Position> {
        let width = self.width;
        let height = self.height;
        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.lost ||
            self.open_case.contains(&position) ||
            self.flagged.contains(&position) {
            return None;
        }
        self.open_case.insert(position);
        let is_mines = self.mines.contains(&position);
        match is_mines {
            true => {
                self.lost = true;
                Some(OpenResult::Mine)
            }
            false => {
                let mine_count = self.neighboring_mines(position);
                if mine_count == 0 {
                    for neighbor in self.iter_neighbors(position) {
                        self.open(neighbor);
                    }
                }
                Some(OpenResult::NoMine(0))
            }
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.lost || self.open_case.contains(&pos) {
            return;
        }
        match self.flagged.contains(&pos) {
            true => {
                self.flagged.remove(&pos);
            }
            false => {
                self.flagged.insert(pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn it_works() {
        let mut ms = Minesweeper::new(10, 10, 3);
        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        println!("{}", ms);
    }
}
