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

type Row = (Cell, Cell, Cell, Cell, Cell);
type Col = Row;

#[derive(Default)]
struct Board {
    cols: (Col, Col, Col, Col, Col),
    rows: (Row, Row, Row, Row, Row),
}

struct Game {
    boards: Vec<Board>,
    draw: Vec<i32>,
}
