use std::str::FromStr;


#[derive(Debug, PartialEq)]
enum Direction {
    North, NorthWest, NorthEast, South, SouthWest, SouthEast
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n"  => Ok(Direction::North),
            "nw" => Ok(Direction::NorthWest),
            "ne" => Ok(Direction::NorthEast),
            "s"  => Ok(Direction::South),
            "sw" => Ok(Direction::SouthWest),
            "se" => Ok(Direction::SouthEast),
            _ => Err(()),
        }
    }
}


#[derive(Debug, PartialEq)]
struct Path {
    steps: Vec<Direction>,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path { steps: try!(s.split(',').map(str::parse).collect()) })
    }
}

impl Path {
    /// Returns the direct distance between start and end
    fn distance(&self) -> usize {
        Self::direct_distance(&self.steps)
    }

    /// Returns the furthest direct distance ever reached
    fn furthest_distance(&self) -> usize {
        (1..self.steps.len()).map(|i|
            Self::direct_distance(&self.steps[..i])
        ).max().unwrap_or(0)
    }

    /// Returns the direct distance between start and end for the given steps
    fn direct_distance(steps: &[Direction]) -> usize {
        let (q, r): (isize, isize) = steps.iter().fold((0, 0), |(q, r), step| {
            match *step {
                Direction::North     => (q, r-1),
                Direction::NorthWest => (q-1, r),
                Direction::NorthEast => (q+1, r-1),
                Direction::South     => (q, r+1),
                Direction::SouthWest => (q-1, r+1),
                Direction::SouthEast => (q+1, r),
            }
        });
        (q.abs() as usize + (q + r).abs() as usize + r.abs() as usize) / 2
    }
}


fn main() {
    let path: Path = include_str!("day11.txt").parse().unwrap();
    println!("Fewest number of steps to reach child: {}", path.distance());
    println!("Furthest number of steps on path: {}", path.furthest_distance());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Path::from_str("ne,sw,se"), Ok(Path { steps: vec![Direction::NorthEast, Direction::SouthWest, Direction::SouthEast] }));
    }

    #[test]
    fn samples1() {
        assert_eq!(Path::from_str("ne,ne,ne").unwrap().distance(), 3);
        assert_eq!(Path::from_str("ne,ne,sw,sw").unwrap().distance(), 0);
        assert_eq!(Path::from_str("ne,ne,s,s").unwrap().distance(), 2);
        assert_eq!(Path::from_str("se,sw,se,sw,sw").unwrap().distance(), 3);
    }

    #[test]
    fn samples2() {
        assert_eq!(Path::from_str("ne,ne,sw,sw").unwrap().furthest_distance(), 2);
    }
}
