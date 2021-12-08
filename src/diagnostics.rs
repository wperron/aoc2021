use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
pub struct Diag {
    gamma: u32,
    epsilon: u32,
}

impl Diag {
    pub fn power_consumption(self) -> u32 {
        self.gamma * self.epsilon
    }
}

impl FromStr for Diag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zeros: Vec<u32> = vec![0, 0, 0, 0, 0];
        let mut ones: Vec<u32> = vec![0, 0, 0, 0, 0];
        for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '0' => zeros[i] += 1,
                    '1' => ones[i] += 1,
                    _ => {}
                }
            }
        }

        let mut gamma: u32 = 0;
        for (z, o) in zeros.iter().zip(ones) {
            gamma = gamma << 1;
            if &o > z {
                gamma += 1;
            }
        }
        let epsilon = !gamma;
        Ok(Self { gamma, epsilon })
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let diag = Diag::from_str(data).unwrap();
        println!("{:?}", diag);
        assert_eq!(diag.gamma, 22);
        assert_eq!(diag.epsilon, 9);
        assert_eq!(diag.power_consumption(), 198);
    }
}
