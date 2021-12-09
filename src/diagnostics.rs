use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
pub struct Diag<const N: u32> {
    orig: String,
    gamma: u32,
    epsilon: u32,
    o2_rating: u32,
}

impl<const N: u32> Diag<N> {
    pub fn power_consumption(self) -> u32 {
        self.gamma * self.epsilon
    }
}

impl<const N: u32> FromStr for Diag<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let owned = String::from(s);
        let cap: usize = N.try_into()?;

        let mut bit_count: Vec<(u32, u32)> = Vec::with_capacity(cap);
        bit_count.resize(cap, (0, 0));
        for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                match c {
                    '0' => bit_count[i].0 += 1,
                    '1' => bit_count[i].1 += 1,
                    _ => {}
                }
            }
        }

        // aka: most common bits.
        let mut gamma: u32 = 0;
        for (z, o) in bit_count {
            gamma = gamma << 1;
            if o > z {
                gamma += 1;
            }
        }

        let mut o2_rating = 0;

        // The `!` bitwise operator flips each bit in the integer, the `&` operation
        // then makes sure that we're flipping all the extra unnecessary bits back
        // zero since we're using u32 but are only concerned about the smallest N bits.
        // aka: least common bits.
        let epsilon = !gamma & ((2 as u32).pow(N) - 1);
        Ok(Self {
            orig: owned,
            gamma,
            epsilon,
            o2_rating,
        })
    }
}

#[cfg(test)]
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
        let diag = Diag::<5>::from_str(data).unwrap();
        println!("{:?}", diag);
        assert_eq!(diag.gamma, 22);
        assert_eq!(diag.epsilon, 9);
        assert_eq!(diag.power_consumption(), 198);
    }
}
