use std::str::FromStr;


/// The list of instructions
#[derive(Debug, PartialEq)]
struct Instructions {
    /// Vector of jump offsets
    jumps: Vec<i32>,
}

impl FromStr for Instructions {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instructions { jumps: try!(s.lines().map(str::parse).collect()) })
    }
}

impl Instructions {
    /// Returns an iterator for executing the instructions
    fn exec(&self) -> Executor {
        Executor { instructions: self, stranger: false, offsets: self.jumps.iter().map(|_| 0).collect(), current: 0 }
    }

    /// Returns an iterator for executing the instructions even stranger
    fn stranger_exec(&self) -> Executor {
        Executor { instructions: self, stranger: true, offsets: self.jumps.iter().map(|_| 0).collect(), current: 0 }
    }
}


/// Executor for instructions
#[derive(Debug)]
struct Executor<'a> {
    /// Instructions (jump offsets)
    instructions: &'a Instructions,
    /// Flag for even stranger execution
    stranger: bool,
    /// Vector of additional jump offsets
    offsets: Vec<i32>,
    /// Pointer to current instruction
    current: i32,
}

impl<'a> Iterator for Executor<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 0 && self.current < self.instructions.jumps.len() as i32 {
            let ip = self.current;
            let jump_offset = self.instructions.jumps[self.current as usize] + self.offsets[self.current as usize];
            self.offsets[ip as usize] += if self.stranger && jump_offset >= 3 { -1 } else { 1 };
            self.current += jump_offset;
            Some(ip)
        } else {
            None
        }
    }
}


fn main() {
    let instructions: Instructions = include_str!("day05.txt").parse().unwrap();
    println!("Number of steps to strangely escape: {}", instructions.exec().count());
    println!("Number of steps to escape even stranger: {}", instructions.stranger_exec().count());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Instructions::from_str("0\n3\n0\n1\n-3"), Ok(Instructions { jumps: vec![0, 3, 0, 1, -3] }));
    }

    #[test]
    fn samples1() {
        let instructions = Instructions::from_str("0\n3\n0\n1\n-3").unwrap();
        assert_eq!(instructions.exec().collect::<Vec<_>>(), vec![0, 0, 1, 4, 1]);
    }

    #[test]
    fn samples2() {
        let instructions = Instructions::from_str("0\n3\n0\n1\n-3").unwrap();
        assert_eq!(instructions.stranger_exec().collect::<Vec<_>>(), vec![0, 0, 1, 4, 1, 3, 4, 2, 2, 3]);
    }
}
