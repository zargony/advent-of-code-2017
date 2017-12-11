use std::str::FromStr;


/// Memory, grouped into banks
#[derive(Debug, Clone, PartialEq)]
struct Memory {
    /// Vector of banks with number of blocks in it
    banks: Vec<u32>,
}

impl FromStr for Memory {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Memory { banks: try!(s.split_whitespace().map(str::parse).collect()) })
    }
}

impl Memory {
    /// Redistributes the largest bank
    fn redistribute(&mut self) {
        if let Some(&max_n) = self.banks.iter().max() {
            let pos = self.banks.iter().position(|n| *n == max_n).unwrap();
            self.banks[pos] = 0;
            let len = self.banks.len();
            for i in 0..(max_n as usize) {
                self.banks[(pos + i + 1) % len] += 1;
            }
        }
    }

    /// Returns an iterator that redistributes all banks until a loop is detected
    fn iter_redist(&self) -> Redistribute {
        Redistribute { history: vec![self.clone()], done: false, dup_distance: None }
    }
}


/// Redistribution iterator
#[derive(Debug, Clone)]
struct Redistribute {
    /// Previous redistributions
    history: Vec<Memory>,
    /// Done flag
    done: bool,
    /// Distance of duplicate results (after done)
    dup_distance: Option<usize>,
}

impl Iterator for Redistribute {
    type Item = Memory;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            let mut m = self.history.last().unwrap().clone();
            m.redistribute();
            if let Some(i) = self.history.iter().position(|mm| *mm == m) {
                self.done = true;
                self.dup_distance = Some(self.history.len() - i);
            } else {
                self.history.push(m.clone());
            }
            Some(m)
        } else {
            None
        }
    }
}


fn main() {
    let memory: Memory = include_str!("day06.txt").parse().unwrap();
    let mut it = memory.iter_redist();
    let mut count = 0;
    while let Some(_) = it.next() { count += 1; }
    println!("Number of redistribution cycles: {}", count);
    println!("Distance of duplication after redistribtion: {}", it.dup_distance.unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Memory::from_str("0\t2\t7\t0"), Ok(Memory { banks: vec![0, 2, 7, 0] }));
    }

    #[test]
    fn samples() {
        let memory = Memory::from_str("0\t2\t7\t0").unwrap();
        let mut it = memory.iter_redist();
        assert_eq!(it.next(), Some(Memory { banks: vec![2, 4, 1, 2] }));
        assert_eq!(it.next(), Some(Memory { banks: vec![3, 1, 2, 3] }));
        assert_eq!(it.next(), Some(Memory { banks: vec![0, 2, 3, 4] }));
        assert_eq!(it.next(), Some(Memory { banks: vec![1, 3, 4, 1] }));
        assert_eq!(it.next(), Some(Memory { banks: vec![2, 4, 1, 2] }));
        assert_eq!(it.next(), None);
        assert_eq!(it.dup_distance, Some(4));
    }
}
