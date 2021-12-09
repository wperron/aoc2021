use std::str::FromStr;

enum CellState {
    Checked,
    Unchecked,
}

impl Default for CellState {
    fn default() -> Self {
        Self::Unchecked
    }
}

#[derive(Default)]
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

type Row<const N: usize> = [Cell; N];
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

struct Game<const N: usize> {
    boards: Vec<Board<N>>,
    draw: Vec<i32>,
}
