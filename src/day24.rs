#[macro_use]
extern crate nom;

use std::{cmp, fmt};
use std::str::FromStr;
use nom::digit;


#[derive(Debug, Clone)]
struct Component {
    port_a: u8,
    port_b: u8,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.port_a, self.port_b)
    }
}

impl FromStr for Component {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(number<&str, u8>, map_res!(digit, str::parse));
        complete!(s, do_parse!(
            a: number >> tag!("/") >> b: number >> (Component { port_a: a, port_b: b })
        )).to_result()
    }
}

impl Component {
    /// Strength of the component
    fn strength(&self) -> u32 {
        self.port_a as u32 + self.port_b as u32
    }
}


#[derive(Debug)]
struct ComponentList(Vec<Component>);

impl FromStr for ComponentList {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ComponentList(try!(s.lines().map(str::parse).collect())))
    }
}

impl ComponentList {
    /// Iterator for building bridges
    fn bridge(&self) -> Bridge {
        Bridge { components: &self.0, placement: vec![], done: false }
    }

    /// Length of the component list
    fn length(&self) -> usize {
        self.0.len()
    }

    /// Strength of the component list
    fn strength(&self) -> u32 {
        self.0.iter().map(Component::strength).sum()
    }

    /// Compares component lists by strength
    fn cmp_strength(&self, other: &Self) -> cmp::Ordering {
        self.strength().cmp(&other.strength())
    }

    /// Compares component lists by length and strength
    fn cmp_length_strength(&self, other: &Self) -> cmp::Ordering {
        match self.length().cmp(&other.length()) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Equal => self.cmp_strength(other),
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        }
    }
}


#[derive(Debug)]
struct Bridge<'a> {
    components: &'a [Component],
    placement: Vec<(usize, bool)>,
    done: bool,
}

impl<'a> fmt::Display for Bridge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &(i, _) in &self.placement {
            try!(write!(f, "{}--", self.components[i]));
        }
        Ok(())
    }
}

impl<'a> Iterator for Bridge<'a> {
    type Item = ComponentList;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }
        let mut i = 0;
        while i < self.components.len() {
            if let Some(f) = self.can_place(i) {
                self.placement.push((i, f));
                return Some(self.component_list());
            }
            i += 1;
        }
        while !self.placement.is_empty() {
            let mut i = self.placement.pop().unwrap().0 + 1;
            while i < self.components.len() {
                if let Some(f) = self.can_place(i) {
                    self.placement.push((i, f));
                    return Some(self.component_list());
                }
                i += 1;
            }
        }
        self.done = true;
        None
    }
}

impl<'a> Bridge<'a> {
    /// Returns the port the next component needs to match
    fn next_port(&self) -> u8 {
        self.placement.last().map(|&(i, f)|
            if f {
                self.components[i].port_a
            } else {
                self.components[i].port_b
            }
        ).unwrap_or(0)
    }

    /// Returns whether the given component can be placed
    fn can_place(&self, idx: usize) -> Option<bool> {
        if self.placement.iter().any(|&(i, _)| i == idx) {
            return None;
        }
        let port = self.next_port();
        if self.components[idx].port_a == port {
            Some(false)
        } else if self.components[idx].port_b == port {
            Some(true)
        } else {
            None
        }
    }

    /// Components of the current bridge
    fn component_list(&self) -> ComponentList {
        ComponentList(self.placement.iter().map(|&(i, _)| self.components[i].clone()).collect())
    }
}


fn main() {
    let components: ComponentList = include_str!("day24.txt").parse().unwrap();
    println!("Strength of strongest bridge: {}", components.bridge().max_by(ComponentList::cmp_strength).unwrap().strength());
    println!("Strength of longest bridge: {}", components.bridge().max_by(ComponentList::cmp_length_strength).unwrap().strength());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert!(ComponentList::from_str(include_str!("day24.txt")).is_ok());
    }

    #[test]
    fn samples1() {
        let components = ComponentList::from_str("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10\n").unwrap();
        assert_eq!(components.bridge().max_by(ComponentList::cmp_strength).unwrap().strength(), 31);
    }

    #[test]
    fn samples2() {
        let components = ComponentList::from_str("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10\n").unwrap();
        assert_eq!(components.bridge().max_by(ComponentList::cmp_length_strength).unwrap().strength(), 19);
    }
}
