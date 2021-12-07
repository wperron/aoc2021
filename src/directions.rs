use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct Coord(pub i32, pub i32);

pub struct Vector {
    dir: Direction,
    l: i32,
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");
        let dir = parts.next().expect("expected direction at first column");
        let dir = Direction::from_str(dir)?;
        let l = parts.next().expect("").parse::<i32>()?;
        Ok(Self { dir, l })
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(anyhow!("unknown direction")),
        }
    }
}

pub struct Move {
    directions: Vec<Vector>,
    start: Coord,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            directions: Default::default(),
            start: Coord(0, 0),
        }
    }
}

impl From<Vec<Vector>> for Move {
    fn from(v: Vec<Vector>) -> Self {
        let mut m = Move::default();
        m.directions = v;
        m
    }
}

impl Move {
    pub fn follow(self) -> Coord {
        let mut coord = self.start.clone();
        for v in self.directions {
            match v.dir {
                Direction::Forward => coord.0 += v.l,
                Direction::Up => coord.1 -= v.l,
                Direction::Down => coord.1 += v.l,
            }
        }
        coord
    }
}
