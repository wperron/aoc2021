mod directions;

use std::str::FromStr;

use directions::{Coord, Vector};

use anyhow::Result;

fn main() {
    const DATA: &str = include_str!("./inputs/day2task1.txt");
    let lines = DATA.lines();
    let mut moves: Vec<Vector> = vec![];
    for l in lines {
        let v = Vector::from_str(l).unwrap();
        moves.push(v);
    }

    let start = Coord(0, 0);
    let end = directions::follow(moves, start).unwrap();
    println!("{}, {}", end.0, end.1);
}

#[allow(dead_code)]
fn str_to_num_vec(s: &str) -> Result<Vec<i32>> {
    let mut res: Vec<i32> = vec![];
    for l in s.lines() {
        let n = l.parse::<i32>()?;
        res.push(n);
    }
    Ok(res)
}

/// Day 1 - part 1
#[allow(dead_code)]
fn num_increase(data: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for curr in data.windows(2) {
        if curr[0] < curr[1] {
            sum += 1;
        }
    }
    sum
}

/// Day 1 - part 2
#[allow(dead_code)]
fn moving_window(data: Vec<i32>, size: usize) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for w in data.windows(size) {
        res.push(w.iter().sum());
    }
    res
}
