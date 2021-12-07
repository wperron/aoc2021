use std::str::FromStr;

use anyhow::Result;

pub struct Diag {
    gamma: i32,
    epsilon: i32,
}

impl Diag {
    pub fn power_consumption(self) -> i32 {
        self.gamma * self.epsilon
    }
}

impl FromStr for Diag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_lines = 0;
        let mut sums: (u8, u8, u8, u8, u8) = (0, 0, 0, 0, 0);
        for l in s.lines() {
            let num = u8::from_str_radix(l, 10)?;
        }
        todo!()
    }
}
