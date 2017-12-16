#![cfg_attr(feature = "nightly", feature(test))]

#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;
use nom::{digit, anychar};


#[derive(Debug, PartialEq)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(number<&str, usize>, map_res!(digit, str::parse));
        complete!(s, alt!(
            do_parse!(
                tag!("s") >> size: number >> (Move::Spin(size))
            ) | do_parse!(
                tag!("x") >> pos1: number >> tag!("/") >> pos2: number >> (Move::Exchange(pos1, pos2))
            ) | do_parse!(
                tag!("p") >> name1: anychar >> tag!("/") >> name2: anychar >> (Move::Partner(name1, name2))
            )
        )).to_result()
    }
}

impl Move {
    /// Applies the move to the given group of dancers
    fn apply(&self, dancers: &mut [char]) {
        let len = dancers.len();
        match *self {
            Move::Spin(a) => {
                for (i, &d) in dancers[len-a..].to_owned().iter().chain(dancers[..len-a].to_owned().iter()).enumerate() {
                    dancers[i] = d;
                }
            },
            Move::Exchange(a, b) => {
                dancers.swap(a, b);
            },
            Move::Partner(a, b) => {
                match (dancers.iter().position(|&d| d==a), dancers.iter().position(|&d| d==b)) {
                    (Some(a), Some(b)) => dancers.swap(a, b),
                    _ => panic!("Unknown dancer to partner with"),
                }
            },
        }
    }
}


#[derive(Debug, PartialEq)]
struct Dance {
    moves: Vec<Move>,
}

impl FromStr for Dance {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Dance { moves: try!(s.split(',').map(str::parse).collect()) })
    }
}

impl Dance {
    /// Perform the dance
    fn perform(&self, group_size: usize, iterations: usize) -> String {
        let mut cache: HashMap<Vec<char>, Vec<char>> = HashMap::new();
        let mut dancers: Vec<char> = (0..group_size).map(|i| ('a' as usize + i) as u8 as char).collect();
        for _ in 0..iterations {
            if let Some(result) = cache.get(&dancers) {
                dancers = result.clone();
                continue;
            }
            let input = dancers.clone();
            for moove in &self.moves {
                moove.apply(&mut dancers);
            }
            cache.insert(input, dancers.clone());
        }
        dancers.iter().collect()
    }
}


fn main() {
    let dance: Dance = include_str!("day16.txt").parse().unwrap();
    println!("Order of programs after 1 dance: {}", dance.perform(16, 1));
    println!("Order of programs after 1,000,000,000 dances: {}", dance.perform(16, 1_000_000_000));
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    extern crate test;

    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Move::from_str("s1"), Ok(Move::Spin(1)));
        assert_eq!(Move::from_str("x3/4"), Ok(Move::Exchange(3, 4)));
        assert_eq!(Move::from_str("pe/b"), Ok(Move::Partner('e', 'b')));
        assert_eq!(Dance::from_str("s1,x3/4,pe/b"), Ok(Dance { moves: vec![Move::Spin(1), Move::Exchange(3, 4), Move::Partner('e', 'b')] }));
    }

    #[test]
    fn samples() {
        let dance = Dance::from_str("s1,x3/4,pe/b").unwrap();
        assert_eq!(dance.perform(5, 1), "baedc");
        assert_eq!(dance.perform(5, 2), "ceadb");
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn benchmark_simple_dance(b: &mut test::Bencher) {
        let dance: Dance = include_str!("day16.txt").parse().unwrap();
        b.iter(|| {
            dance.perform(16, 1_000_000)
        })
    }
}
