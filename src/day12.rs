#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::str::FromStr;
use nom::digit;


#[derive(Debug, PartialEq)]
struct Program {
    id: u32,
    pipes: Vec<u32>,
}

impl FromStr for Program {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(number<&str, u32>, map_res!(digit, str::parse));
        complete!(s, do_parse!(
            id: ws!(number) >>
            tag!("<->") >>
            pipes: ws!(separated_list_complete!(tag!(","), ws!(number))) >>
            (Program { id: id, pipes: pipes })
        )).to_result()
    }
}


#[derive(Debug, PartialEq)]
struct Village {
    programs: Vec<Program>,
}

impl FromStr for Village {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Village { programs: try!(s.lines().map(str::parse).collect()) })
    }
}

impl Village {
    /// Get program with the given id
    fn program(&self, id: u32) -> Option<&Program> {
        self.programs.iter().find(|p| p.id == id)
    }

    /// Get a set of all program ids that are in the group of the given program
    fn group_of_program(&self, id: u32) -> HashSet<u32> {
        let mut set = HashSet::new();
        let mut ids = vec![id];
        while let Some(id) = ids.pop() {
            if set.insert(id) {
                ids.extend(&self.program(id).unwrap().pipes);
            }
        }
        set
    }

    /// Count number of separated groups
    fn count_groups(&self) -> usize {
        let mut set = HashSet::new();
        let mut count = 0;
        for p in &self.programs {
            if !set.contains(&p.id) {
                set.extend(self.group_of_program(p.id));
                count += 1;
            }
        }
        count
    }

}


fn main() {
    let village: Village = include_str!("day12.txt").parse().unwrap();
    println!("Number of programs in group of program 0: {}", village.group_of_program(0).len());
    println!("Number of disjoint groups: {}", village.count_groups());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Program::from_str("2 <-> 0, 3, 4"), Ok(Program { id: 2, pipes: vec![0, 3, 4] }));
        assert_eq!(Village::from_str("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5"),
            Ok(Village { programs: vec![
                Program { id: 0, pipes: vec![2] },
                Program { id: 1, pipes: vec![1] },
                Program { id: 2, pipes: vec![0, 3, 4] },
                Program { id: 3, pipes: vec![2, 4] },
                Program { id: 4, pipes: vec![2, 3, 6] },
                Program { id: 5, pipes: vec![6] },
                Program { id: 6, pipes: vec![4, 5] },
            ]}));
    }

    #[test]
    fn samples1() {
        let village = Village::from_str("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5").unwrap();
        assert_eq!(village.group_of_program(0), vec![0, 2, 3, 4, 5, 6].into_iter().collect());
    }

    #[test]
    fn samples2() {
        let village = Village::from_str("0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5").unwrap();
        assert_eq!(village.count_groups(), 2);
    }
}
