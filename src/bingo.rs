use std::str::FromStr;

#[derive(Copy, Clone)]
enum CellState {
    Checked,
    Unchecked,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Unchecked
    }
}

#[derive(Default, Copy, Clone)]
struct Cell {
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

#[derive(Clone, Copy)]
struct Row<const N: usize> {
    inner: [Cell; N],
}

impl<const N: usize> Default for Row<N> {
    fn default() -> Self {
        Self {
            inner: [Default::default(); N],
        }
    }
}

type Col<const N: usize> = Row<N>;

struct Board<const N: usize> {
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

#[derive(Default)]
struct Game<const N: usize> {
    boards: Vec<Board<N>>,
    draw: Vec<i32>,
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

        let mut boards: Vec<Board> = vec![];
        let mut cols: [Col; N] = [Default::default(); N];
        let mut rows: [Row; N] = [Default::default(); N];
        for line in lines {
            // The input contains a few empty lines, this check just skips those.
            if line.trim().len() == 0 {
                continue;
            }

            for num in line.split(" ") {
                let num = i32::from_str_radix(num.trim(), 10)?;
            }
        }

        Ok(Self { draw, boards })
    }
}
