use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Checked,
    Unchecked,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Unchecked
    }
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Checked => write!(f, "{}", "✅"),
            CellState::Unchecked => write!(f, "{}", "⬜"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    val: i32,
    state: CellState,
}

impl From<i32> for Cell {
    fn from(i: i32) -> Self {
        Self {
            val: i,
            state: Default::default(),
        }
    }
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = i32::from_str_radix(s, 10)?;
        Ok(Cell::from(n))
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.val, self.state)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Row<const N: usize> {
    inner: [Cell; N],
}

impl<const N: usize> Default for Row<N> {
    fn default() -> Self {
        Self {
            inner: [Default::default(); N],
        }
    }
}

impl<const N: usize> Row<N> {
    fn complete(self) -> bool {
        self.inner
            .iter()
            .fold(true, |comp, cell| comp && cell.state == CellState::Checked)
    }
}

pub type Col<const N: usize> = Row<N>;

#[derive(Clone, Copy, Debug)]
pub struct Board<const N: usize> {
    // TODO(wperron) add a HashMap<i32, Cell> to make lookup on the `draw` fn
    // quicker
    cols: [Col<N>; N],
    rows: [Row<N>; N],
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Self {
            cols: [Default::default(); N],
            rows: [Default::default(); N],
        }
    }
}

impl<const N: usize> Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows {
            for cell in row.inner {
                write!(f, "{}\t", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<const N: usize> Board<N> {
    fn draw(&mut self, n: i32) {
        for r in &mut self.rows {
            for mut cell in &mut r.inner {
                if cell.val == n {
                    cell.state = CellState::Checked;
                }
            }
        }
    }

    fn complete(self) -> bool {
        for row in self.rows {
            if row.complete() {
                return true;
            }
        }

        for col in self.cols {
            if col.complete() {
                return true;
            }
        }

        false
    }

    pub fn sum_unchecked(self) -> i32 {
        self.rows.into_iter().fold(0, |sum, r| {
            sum + r.inner.into_iter().fold(0, |sum, cell| match cell.state {
                CellState::Checked => sum,
                CellState::Unchecked => sum + cell.val,
            })
        })
    }
}

#[derive(Default, Debug)]
pub struct Game<const N: usize> {
    boards: Vec<Board<N>>,
    draw: Vec<i32>,
}

impl<const N: usize> Game<N> {
    pub fn run(mut self) -> Option<(Board<N>, Vec<i32>)> {
        let mut draw_iter = self.draw.iter();
        for _ in 0..5 {
            let curr = draw_iter
                .next()
                .expect("expected at least 5 numbers in draw");

            for b in &mut self.boards {
                b.draw(*curr);
            }
        }

        for b in &self.boards {
            if b.complete() {
                return Some((b.clone(), self.draw[0..5].into()));
            }
        }

        let mut i = 6;
        for next in draw_iter {
            for b in &mut self.boards {
                b.draw(*next);
            }

            for b in &self.boards {
                if b.complete() {
                    return Some((b.clone(), self.draw[0..i].into()));
                }
            }
            i += 1;
        }

        None
    }
}

impl<const N: usize> FromStr for Game<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let draw: Vec<i32> = lines
            .next()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidData))?
            .split(',')
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect();

        // Consume the first empty line after the draw.
        let _ = lines.next();

        let mut boards: Vec<Board<N>> = vec![];
        let mut curr_board: Board<N> = Default::default();
        let mut i = 0;
        for line in lines {
            let line = line.trim();
            if line.len() != 0 {
                for (j, num) in line.split_ascii_whitespace().enumerate() {
                    let cell = Cell::from_str(num.trim())?;
                    curr_board.rows[i].inner[j] = cell;
                    curr_board.cols[j].inner[i] = cell;
                }
                i += 1;
            } else {
                // an empty line denotes the end of one board and the subsequent
                // beginning of another.
                boards.push(curr_board.clone());
                curr_board = Default::default();
                i = 0;
            }
        }

        // push the last board
        boards.push(curr_board);

        Ok(Self { draw, boards })
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_from_str() {
        let val = "1";
        let cell = Cell::from_str(val).unwrap();
        assert_eq!(cell.val, 1);
        assert_eq!(cell.state, CellState::Unchecked);

        let raw = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";
        let game: Game<5> = Game::from_str(raw).unwrap();
        assert_eq!(game.boards.len(), 1);
        assert_eq!(game.draw.len(), 27);
        let board = game.boards[0];
        assert_eq!(board.rows[1].inner[2], board.cols[2].inner[1]);

        let raw = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let game: Game<5> = Game::from_str(raw).unwrap();
        assert_eq!(game.boards.len(), 3);
        assert_eq!(game.draw.len(), 27);
        let board = game.boards[0];
        assert_eq!(board.rows[1].inner[2], board.cols[2].inner[1]);

        let board = game.boards[1];
        assert_eq!(board.rows[1].inner[2], board.cols[2].inner[1]);

        let board = game.boards[2];
        assert_eq!(board.rows[1].inner[2], board.cols[2].inner[1]);
    }

    #[test]
    fn test_run() {
        let raw = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let game: Game<5> = Game::from_str(raw).unwrap();
        let winner = game.run();
        assert!(winner.is_some());
        let winner = winner.unwrap();
        assert_eq!(winner.0.cols[0].inner[0].val, 14);
        assert_eq!(winner.0.sum_unchecked(), 188);
        assert_eq!(winner.1.len(), 12);
        assert_eq!(*winner.1.last().unwrap(), 24);
    }
}
