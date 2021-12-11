mod bingo;
mod diagnostics;
mod submarine;

use std::str::FromStr;

use submarine::Vector;

use anyhow::Result;

use crate::bingo::Game;

fn main() {
    const DATA: &str = include_str!("./inputs/day4task1.txt");

    let game: Game<5> = Game::from_str(DATA).unwrap();
    let (winner, win_draw) = game.run().unwrap();
    println!("{:?}", winner.sum_unchecked() * *win_draw.last().unwrap());
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

/// Day 2 - part 1
#[allow(dead_code)]
fn parse_directions(data: &str) -> Vec<Vector> {
    let mut moves: Vec<Vector> = vec![];
    for l in data.lines() {
        let v = Vector::from_str(l).unwrap();
        moves.push(v);
    }
    moves
}

/// Day 2 - part 1
#[allow(dead_code)]
fn calc_depth_product(x: i32, y: i32) -> i32 {
    x * y
}
