#![cfg_attr(feature = "nightly", feature(test))]

#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::digit;


#[derive(Debug, PartialEq)]
struct Layer {
    depth: u32,
    range: u32,
}

impl FromStr for Layer {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(number<&str, u32>, map_res!(digit, str::parse));
        complete!(s, do_parse!(
            depth: ws!(number) >>
            tag!(":") >>
            range: ws!(number) >>
            (Layer { depth: depth, range: range })
        )).to_result()
    }
}


#[derive(Debug, PartialEq)]
struct Firewall {
    layers: Vec<Layer>,
}

impl FromStr for Firewall {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Firewall { layers: try!(s.lines().map(str::parse).collect()) })
    }
}

impl Firewall {
    /// Total depth of firewall
    fn depth(&self) -> u32 {
        self.layers.iter().map(|l| l.depth).max().unwrap_or(0)
    }

    /// Severity of a packet travelling through the top of the firewall. None if uncaught
    fn severity_with_delay(&self, start_delay: u32) -> Option<u32> {
        (0 .. self.depth() + 1).map(|t|
            self.layers.iter().find(|l| l.depth == t).and_then(|layer|
                match (start_delay + t) % (2 * layer.range - 2) {
                    0 => Some(layer.depth * layer.range),
                    _ => None,
                 }
            )
        ).fold(None, |sum, s|
            sum.map(|x| x + s.unwrap_or(0)).or(s)
        )
    }

    /// Severity of a packet travelling through the top of the firewall
    fn severity(&self) -> u32 {
        self.severity_with_delay(0).unwrap_or(0)
    }

    /// Returns the delay required to pass the firewall without being caught
    fn required_delay_for_passing(&self) -> u32 {
        (0..).find(|&d| self.severity_with_delay(d).is_none()).unwrap()
    }
}


fn main() {
    let firewall: Firewall = include_str!("day13.txt").parse().unwrap();
    println!("Severity of packet trip: {}", firewall.severity());
    println!("Delay required to pass the firewall: {}", firewall.required_delay_for_passing());
}


#[cfg(test)]
mod tests {
    #[cfg(feature = "nightly")]
    extern crate test;

    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Firewall::from_str("0: 3\n1: 2\n4: 4\n6: 4"), Ok(Firewall { layers: vec![
            Layer { depth: 0, range: 3 },
            Layer { depth: 1, range: 2 },
            Layer { depth: 4, range: 4 },
            Layer { depth: 6, range: 4 },
        ] }));
    }

    #[test]
    fn samples() {
        let firewall = Firewall::from_str("0: 3\n1: 2\n4: 4\n6: 4").unwrap();
        assert_eq!(firewall.severity(), 24);
        assert_eq!(firewall.required_delay_for_passing(), 10);
    }

    #[cfg(feature = "nightly")]
    #[bench]
    fn benchmark_required_delay_for_passing(b: &mut test::Bencher) {
        let firewall: Firewall = include_str!("day13.txt").parse().unwrap();
        b.iter(|| {
            firewall.required_delay_for_passing()
        })
    }
}
