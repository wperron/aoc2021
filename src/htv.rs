/// Hypothermal Vents navigation system
use std::{cmp::max, cmp::min, collections::HashMap, fmt::Display, ops::Range, str::FromStr};

use anyhow::Result;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(coord: (i32, i32)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
        }
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts
            .next()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        let x = i32::from_str_radix(x, 10)?;
        let y = parts
            .next()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        let y = i32::from_str_radix(y, 10)?;
        Ok(Self::from((x, y)))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    from: Coord,
    to: Coord,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let from = parts
            .next()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        let from = Coord::from_str(from)?;
        let to = parts
            .next()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?;
        let to = Coord::from_str(to)?;

        Ok(Self { from, to })
    }
}

impl Vector {
    /// Expands a Vector v into a list of Coordiates covered by the Vector.
    pub fn expand(v: Vector) -> Vec<Coord> {
        if v.from == v.to {
            return vec![v.from];
        }

        // Moving in diagonals is currently not supported, so if the Vector
        // doesn't strictly move on the X or Y axis, return an empty list.
        let mut coords = vec![];
        if v.from.x == v.to.x {
            let x = v.from.x;
            let lower = min(v.from.y, v.to.y);
            let higher = max(v.from.y, v.to.y);
            for y in lower..higher + 1 {
                coords.push(Coord { x, y });
            }
        } else if v.from.y == v.to.y {
            let y = v.from.y;
            let lower = min(v.from.x, v.to.x);
            let higher = max(v.from.x, v.to.x);
            for x in lower..higher + 1 {
                coords.push(Coord { x, y });
            }
        } else if (v.from.y - v.to.y).abs() == (v.from.x - v.to.x).abs() {
            // "perfect" diagonal
            let mut range_x = v.from.x..v.to.x + 1;
            if range_x.is_empty() {
                range_x = Range::from((v.to.x..v.from.x + 1).rev());
            }

            let mut range_y = v.from.y..v.to.y + 1;
            if range_y.is_empty() {
                range_y = (v.to.y..v.from.y + 1).rev();
            }

            coords = range_x
                .zip(range_y)
                .map(|(x, y)| Coord::from((x, y)))
                .collect();
        } else {
            println!("{} is a diagonal", v);
        }

        coords
    }
}

pub struct CartographicMap {
    pub inner: HashMap<Coord, i32>,
}

impl Display for CartographicMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width: usize = self
            .inner
            .clone()
            .into_keys()
            .max_by(|a, b| a.x.cmp(&b.x))
            .unwrap_or_default()
            .x as usize;
        let height: usize = self
            .inner
            .clone()
            .into_keys()
            .max_by(|a, b| a.y.cmp(&b.y))
            .unwrap_or_default()
            .y as usize;

        let mut grid = vec![vec![0; width + 1]; height + 1];
        self.inner
            .clone()
            .into_iter()
            .for_each(|(coord, val)| grid[coord.y as usize][coord.x as usize] = val);

        for row in grid {
            for col in row {
                let _ = match col {
                    0 => write!(f, "."),
                    n => write!(f, "{}", n),
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn intersections(vectors: Vec<Vector>) -> HashMap<Coord, i32> {
    let mut map: HashMap<Coord, i32> = HashMap::new();

    for v in vectors {
        for coord in Vector::expand(v) {
            *map.entry(coord).or_insert(0) += 1;
        }
    }

    map
}

pub fn danger_zone(vectors: HashMap<Coord, i32>) -> i32 {
    vectors.into_values().filter(|v| *v > 1).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coord() {
        let raw = "1,3";
        let coord = Coord::from_str(raw);
        assert!(coord.is_ok());
        let coord = coord.unwrap();
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 3);
    }

    #[test]
    fn test_vector() {
        let raw = "1,1 -> 1,5";
        let vector = Vector::from_str(raw);
        assert!(vector.is_ok());
        let vector = vector.unwrap();
        assert_eq!(vector.from, Coord { x: 1, y: 1 });
        assert_eq!(vector.to, Coord { x: 1, y: 5 });
    }

    #[test]
    fn test_negative_vector() {
        let raw = "0,9 -> 0,3";
        let vector = Vector::from_str(raw);
        assert!(vector.is_ok());
        let vector = vector.unwrap();
        assert_eq!(vector.from, Coord { x: 0, y: 9 });
        assert_eq!(vector.to, Coord { x: 0, y: 3 });
    }

    #[test]
    fn test_vector_expand() {
        let v = Vector {
            from: Coord { x: 1, y: 1 },
            to: Coord { x: 1, y: 6 },
        };
        let coords = Vector::expand(v);

        assert_eq!(coords.len(), 6);
        for (i, c) in coords.into_iter().enumerate() {
            assert_eq!(c.x, 1);
            let i: i32 = i.try_into().unwrap();
            assert_eq!(c.y, i + 1);
        }

        let v = Vector {
            from: Coord { x: 1, y: 6 },
            to: Coord { x: 1, y: 1 },
        };
        let coords = Vector::expand(v);

        assert_eq!(coords.len(), 6);
        for (i, c) in coords.into_iter().enumerate() {
            assert_eq!(c.x, 1);
            let i: i32 = i.try_into().unwrap();
            assert_eq!(c.y, i + 1);
        }

        let v = Vector {
            from: Coord { x: 1, y: 1 },
            to: Coord { x: 7, y: 1 },
        };
        let coords = Vector::expand(v);

        assert_eq!(coords.len(), 7);
        for (i, c) in coords.into_iter().enumerate() {
            let i: i32 = i.try_into().unwrap();
            assert_eq!(c.y, 1);
            assert_eq!(c.x, i + 1);
        }

        let v = Vector {
            from: Coord { x: 1, y: 4 },
            to: Coord { x: 7, y: 1 },
        };
        let coords = Vector::expand(v);

        assert_eq!(coords.len(), 0);
    }

    #[test]
    fn test_expand_diagonal() {
        let v = Vector {
            from: Coord { x: 1, y: 5 },
            to: Coord { x: 4, y: 2 },
        };
        let coords = Vector::expand(v);

        assert_eq!(coords.len(), 4);
        let mut iter = coords.into_iter();
        assert_eq!(iter.next().unwrap(), Coord { x: 1, y: 5 });
        assert_eq!(iter.next().unwrap(), Coord { x: 2, y: 4 });
        assert_eq!(iter.next().unwrap(), Coord { x: 3, y: 3 });
        assert_eq!(iter.next().unwrap(), Coord { x: 4, y: 2 });
    }

    #[test]
    fn test_intersection() {
        let vec_a = Vector {
            from: Coord { x: 3, y: 0 },
            to: Coord { x: 3, y: 5 },
        };
        let vec_b = Vector {
            from: Coord { x: 0, y: 3 },
            to: Coord { x: 5, y: 3 },
        };

        let inter = intersections(vec![vec_a, vec_b]);
        // 6 points vertically + 6 points horizontally - 1 common point.
        assert_eq!(inter.len(), 11);
        assert_eq!(inter.into_values().filter(|v| *v > 1).count(), 1);
    }

    #[test]
    fn test_integration() {
        let raw = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let vecs: Vec<Vector> = raw
            .lines()
            .map(|line| Vector::from_str(line).unwrap())
            .collect();

        println!("{:?}", vecs);

        let inters = intersections(vecs);
        assert_eq!(inters.len(), 21);
        assert_eq!(inters.clone().into_values().filter(|v| *v > 1).count(), 5);

        let danger = danger_zone(inters);
        assert_eq!(danger, 5);
    }
}
