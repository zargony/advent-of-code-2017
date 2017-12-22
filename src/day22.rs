use std::collections::HashMap;
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

    fn reverse(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Clean, Weakened, Infected, Flagged,
}

#[derive(Debug)]
struct Cluster {
    states: HashMap<(isize, isize), State>,
}

impl FromStr for Cluster {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut states = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    states.insert((row as isize - height as isize / 2, col as isize - width as isize / 2), State::Infected);
                }
            }
        }
        Ok(Cluster { states: states })
    }
}

impl Cluster {
    fn get(&self, row: isize, col: isize) -> State {
        self.states.get(&(row, col)).map_or(State::Clean, |&s| s)
    }

    fn set(&mut self, row: isize, col: isize, state: State) {
        self.states.insert((row, col), state);
    }

    fn carrier_mut(&mut self) -> Carrier {
        Carrier { cluster: self, row: 0, col: 0, dir: Direction::North }
    }

    fn carrier_advanced_mut(&mut self) -> CarrierAdvanced {
        CarrierAdvanced { cluster: self, row: 0, col: 0, dir: Direction::North }
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
        let infected = match self.cluster.get(self.row, self.col) {
            State::Clean => {
                self.dir = self.dir.left();
                self.cluster.set(self.row, self.col, State::Infected);
                true
            }
            State::Infected => {
                self.dir = self.dir.right();
                self.cluster.set(self.row, self.col, State::Clean);
                false
            },
            State::Weakened => unreachable!(),
            State::Flagged => unreachable!(),
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


#[derive(Debug)]
struct CarrierAdvanced<'a> {
    cluster: &'a mut Cluster,
    row: isize,
    col: isize,
    dir: Direction,
}

impl<'a> Iterator for CarrierAdvanced<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let infected = match self.cluster.get(self.row, self.col) {
            State::Clean => {
                self.dir = self.dir.left();
                self.cluster.set(self.row, self.col, State::Weakened);
                false
            }
            State::Weakened => {
                self.cluster.set(self.row, self.col, State::Infected);
                true
            },
            State::Infected => {
                self.dir = self.dir.right();
                self.cluster.set(self.row, self.col, State::Flagged);
                false
            },
            State::Flagged => {
                self.dir = self.dir.reverse();
                self.cluster.set(self.row, self.col, State::Clean);
                false
            },
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
    let infected = cluster.carrier_mut().take(10_000).filter(|&i| i).count();
    println!("Bursts that cause a node to become infected: {}", infected);

    let mut cluster: Cluster = include_str!("day22.txt").parse().unwrap();
    let infected = cluster.carrier_advanced_mut().take(10_000_000).filter(|&i| i).count();
    println!("Bursts that cause a node to become infected (advanced): {}", infected);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.get(-1, 1), State::Infected);
        assert_eq!(cluster.get(0, -1), State::Infected);
        assert_eq!(cluster.get(0, 0), State::Clean);
    }

    #[test]
    fn samples1a() {
        let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.carrier_mut().take(70).filter(|&i| i).count(), 41);
    }

    #[test]
    fn samples1b() {
        let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.carrier_mut().take(10_000).filter(|&i| i).count(), 5587);
    }

    #[test]
    fn samples2a() {
        let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
        assert_eq!(cluster.carrier_advanced_mut().take(100).filter(|&i| i).count(), 26);
    }

    // #[test]
    // fn samples2b() {
    //     let mut cluster = Cluster::from_str("..#\n#..\n...\n").unwrap();
    //     assert_eq!(cluster.carrier_advanced_mut().take(10_000_000).filter(|&i| i).count(), 2511944);
    // }
}
