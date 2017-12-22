use std::collections::HashSet;
use std::str::FromStr;


#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North, West, South, East,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }
}


#[derive(Debug)]
struct Cluster {
    infected: HashSet<(isize, isize)>,
}

impl FromStr for Cluster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut infected = HashSet::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    infected.insert((row as isize - height as isize / 2, col as isize - width as isize / 2));
                }
            }
        }
        Ok(Cluster { infected: infected })
    }
}

impl Cluster {
    fn carrier_mut(&mut self) -> Carrier {
        Carrier { cluster: self, row: 0, col: 0, dir: Direction::North }
    }
}


#[derive(Debug)]
struct Carrier<'a> {
    cluster: &'a mut Cluster,
    row: isize,
    col: isize,
    dir: Direction,
}

impl<'a> Iterator for Carrier<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let infected = if self.cluster.infected.contains(&(self.row, self.col)) {
            self.dir = self.dir.right();
            self.cluster.infected.remove(&(self.row, self.col));
            false
        } else {
            self.dir = self.dir.left();
            self.cluster.infected.insert((self.row, self.col));
            true
        };
        match self.dir {
            Direction::North => self.row -= 1,
            Direction::West => self.col -= 1,
            Direction::South => self.row += 1,
            Direction::East => self.col += 1,
        }
        Some(infected)
    }
}


fn main() {
    let mut cluster: Cluster = include_str!("day22.txt").parse().unwrap();
    let infected = cluster.carrier_mut().take(10000).filter(|&i| i).count();
    println!("Bursts that cause a node to become infected: {}", infected);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert!(cluster.infected.contains(&(-1, 1)));
        assert!(cluster.infected.contains(&(0, -1)));
        assert!(!cluster.infected.contains(&(0, 0)));
    }

    #[test]
    fn samples1a() {
        let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.carrier_mut().take(70).filter(|&i| i).count(), 41);
    }

    #[test]
    fn samples1b() {
        let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.carrier_mut().take(10000).filter(|&i| i).count(), 5587);
    }
}
