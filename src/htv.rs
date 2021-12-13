/// Hypothermal Vents navigation system
use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
        let mut parts = s.split(",");
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
            return vec![v.from.clone()];
        }

        // Moving in diagonals is currently not supported, so if the Vector
        // doesn't strictly move on the X or Y axis, return an empty list.
        let mut coords = vec![];
        if v.from.x == v.to.x {
            let x = v.from.x;
            for y in v.from.y..v.to.y + 1 {
                coords.push(Coord { x, y });
            }
        } else if v.from.y == v.to.y {
            let y = v.from.y;
            for x in v.from.x..v.to.x + 1 {
                coords.push(Coord { x, y });
            }
        }

        coords
    }
}

pub fn intersections(vectors: Vec<Vector>) -> HashMap<Coord, i32> {
    todo!()
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
}
