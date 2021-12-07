use std::{str::FromStr};

use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Coord (pub i32, pub i32);

pub struct Vector {
    dir: Direction,
    l: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');
        let dir = parts.next().expect("expected direction at first column");
        let dir = serde_json::from_str(dir)?;
        let l = parts.next().expect("").parse::<i32>()?;
        Ok(Self {
            dir,
            l,
        })
    }
}

pub fn follow(directions: Vec<Vector>, start: Coord) -> Result<Coord> {
    let mut coord = start.clone();
    for v in directions {
        match v.dir {
            Direction::Forward => coord.0 += v.l,
            Direction::Up => coord.1 += v.l,
            Direction::Down => coord.1 -= v.l,
        }
    }
    Ok(coord)
}