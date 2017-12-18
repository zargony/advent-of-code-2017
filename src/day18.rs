#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;
use nom::digit;


#[derive(Debug)]
struct RegisterSet {
    regs: HashMap<char, i64>,
}

impl RegisterSet {
    fn new() -> RegisterSet {
        RegisterSet { regs: HashMap::new() }
    }

    fn get(&self, r: char) -> i64 {
        self.regs.get(&r).cloned().unwrap_or(0)
    }

    fn set(&mut self, r: char, v: i64) {
        self.regs.insert(r, v);
    }
}


#[derive(Debug)]
enum Value {
	Register(char),
	Number(i64),
}

impl Value {
    fn get(&self, regs: &RegisterSet) -> i64 {
        match *self {
            Value::Register(r) => regs.get(r),
            Value::Number(n) => n,
        }
    }
}


#[derive(Debug)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value)
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(register<&str, char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
        named!(integer<&str, u64>, map_res!(digit, str::parse));
        named!(number<&str, i64>, alt!(
            map!(preceded!(tag!("-"), integer), |n| -(n as i64)) |
            map!(integer, |n| n as i64)
        ));
        named!(value<&str, Value>, alt!(
            register => { |ch| Value::Register(ch) } |
            number => { |n| Value::Number(n) }
        ));
        complete!(s, alt!(
            do_parse!(tag!("snd") >> x: ws!(value) >> (Instruction::Snd(x))) |
            do_parse!(tag!("set") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Set(x, y))) |
            do_parse!(tag!("add") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Add(x, y))) |
            do_parse!(tag!("mul") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Mul(x, y))) |
            do_parse!(tag!("mod") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Mod(x, y))) |
            do_parse!(tag!("rcv") >> x: ws!(register) >> (Instruction::Rcv(x))) |
            do_parse!(tag!("jgz") >> x: ws!(value) >> y: ws!(value) >> (Instruction::Jgz(x, y)))
        )).to_result()
    }
}


#[derive(Debug)]
enum CoreError {
    OutOfInstructions,
}


#[derive(Debug)]
struct Core {
    code: Vec<Instruction>,
    pc: usize,
    regs: RegisterSet,
    freq: Option<i64>,
}

impl FromStr for Core {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Core {
            code: try!(s.lines().map(str::parse).collect()),
            pc: 0,
            regs: RegisterSet::new(),
            freq: None,
        })
    }
}

impl Core {
    fn step(&mut self) -> Result<(), CoreError> {
        match self.code.get(self.pc) {
            Some(ins) => {
                match ins {
                    &Instruction::Snd(ref v) => {
                        let n = v.get(&self.regs);
                        self.freq = Some(n);
                    },
                    &Instruction::Set(r, ref v) => {
                        let n = v.get(&self.regs);
                        self.regs.set(r, n)
                    },
                    &Instruction::Add(r, ref v) => {
                        let n = self.regs.get(r) + v.get(&self.regs);
                        self.regs.set(r, n);
                    },
                    &Instruction::Mul(r, ref v) => {
                        let n = self.regs.get(r) * v.get(&self.regs);
                        self.regs.set(r, n);
                    },
                    &Instruction::Mod(r, ref v) => {
                        let n = self.regs.get(r) % v.get(&self.regs);
                        self.regs.set(r, n);
                    },
                    &Instruction::Rcv(r) => {
                        if self.regs.get(r) != 0 {
                            self.freq = None;
                        }
                    }
                    &Instruction::Jgz(ref v, ref ofs) => {
                        if v.get(&self.regs) > 0 {
                            let ofs = ofs.get(&self.regs);
                            self.pc = (self.pc as isize + ofs as isize - 1) as usize;
                        }
                    },
                }
                self.pc += 1;
                Ok(())
            }
            None => Err(CoreError::OutOfInstructions),
        }
    }

    fn run_until_recv(&mut self) -> Option<i64> {
        let mut last_freq = None;
        while let Ok(_) = self.step() {
            if self.freq.is_none() && last_freq.is_some() {
                return last_freq;
            }
            last_freq = self.freq;
        }
        None
    }
}


fn main() {
    let mut core: Core = include_str!("day18.txt").parse().unwrap();
    println!("Value of recovered frequency: {}", core.run_until_recv().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert!(Core::from_str(include_str!("day18.txt")).is_ok());
    }

    #[test]
    fn samples1() {
        let mut core = Core::from_str("set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2").unwrap();
        assert_eq!(core.run_until_recv(), Some(4));
    }
}
