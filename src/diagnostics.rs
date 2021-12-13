use std::str::FromStr;

use anyhow::{format_err, Result};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Diag<const N: u32> {
    orig: String,
    gamma: u32,
    epsilon: u32,
    o2_rating: u32,
    co2_rating: u32,
}

impl<const N: u32> Diag<N> {
    #[allow(dead_code)]
    pub fn power_consumption(&self) -> u32 {
        self.gamma * self.epsilon
    }

    #[allow(dead_code)]
    pub fn life_support_rating(&self) -> u32 {
        self.o2_rating * self.co2_rating
    }
}

impl<const N: u32> FromStr for Diag<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let owned = String::from(s);
        let cap_size = N.try_into()?;
        let bit_count = count_bits(owned.lines().map(String::from).collect(), cap_size)?;

        // aka: most common bits.
        let mut gamma: u32 = 0;
        for (z, o) in bit_count.clone() {
            gamma <<= 1;
            if o > z {
                gamma += 1;
            }
        }

        // Get the o2 Rating
        let mut nums: Vec<String> = owned.lines().map(String::from).collect();
        let mut curr_count = bit_count.clone();
        let mut i = 0;
        while nums.len() > 1 {
            println!("{:?}", nums);
            println!("{:?}", curr_count);
            nums = nums
                .into_iter()
                .filter(|s| {
                    let c = s.chars().nth(i).unwrap();
                    if curr_count[i].0 == curr_count[i].1 {
                        return c == '1';
                    }
                    match c {
                        '0' => curr_count[i].0 > curr_count[i].1,
                        '1' => curr_count[i].0 < curr_count[i].1,
                        _ => false,
                    }
                })
                .collect();
            curr_count = count_bits(nums.clone(), cap_size)?;
            i += 1;
        }

        if nums.len() != 1 {
            return Err(format_err!(
                "expected exactly one number for o2 rating, found {}",
                nums.len()
            ));
        }

        let o2_rating = u32::from_str_radix(nums.get(0).unwrap(), 2)?;

        // Get the Co2 Rating, similar to o2
        let mut nums: Vec<String> = owned.lines().map(String::from).collect();
        let mut curr_count = bit_count;
        let mut i = 0;
        while nums.len() > 1 {
            println!("{:?}", nums);
            println!("{:?}", curr_count);
            nums = nums
                .into_iter()
                .filter(|s| {
                    let c = s.chars().nth(i).unwrap();
                    if curr_count[i].0 == curr_count[i].1 {
                        return c == '0';
                    }
                    match c {
                        '0' => curr_count[i].0 < curr_count[i].1,
                        '1' => curr_count[i].0 > curr_count[i].1,
                        _ => false,
                    }
                })
                .collect();
            curr_count = count_bits(nums.clone(), cap_size)?;
            i += 1;
        }

        if nums.len() != 1 {
            return Err(format_err!(
                "expected exactly one number for Co2 rating, found {}",
                nums.len()
            ));
        }

        let co2_rating = u32::from_str_radix(nums.get(0).unwrap(), 2)?;

        // The `!` bitwise operator flips each bit in the integer, the `&` operation
        // then makes sure that we're flipping all the extra unnecessary bits back
        // zero since we're using u32 but are only concerned about the smallest N bits.
        // aka: least common bits.
        let epsilon = !gamma & (2_u32.pow(N) - 1);
        Ok(Self {
            orig: owned,
            gamma,
            epsilon,
            o2_rating,
            co2_rating,
        })
    }
}

fn count_bits(s: Vec<String>, bit_size: usize) -> Result<Vec<(u32, u32)>> {
    let mut bit_count: Vec<(u32, u32)> = Vec::with_capacity(bit_size);
    bit_count.resize(bit_size, (0, 0));
    for line in s {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => bit_count[i].0 += 1,
                '1' => bit_count[i].1 += 1,
                u => return Err(format_err!("unexpected char {}", u)),
            }
        }
    }
    Ok(bit_count)
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
        assert_eq!(diag.o2_rating, 23);
        assert_eq!(diag.co2_rating, 10);
        assert_eq!(diag.power_consumption(), 198);
    }
}
