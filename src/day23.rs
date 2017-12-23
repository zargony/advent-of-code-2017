#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;
use nom::digit;


#[derive(Debug, Clone)]
struct RegisterSet {
    regs: HashMap<char, i64>,
}

impl RegisterSet {
    fn new() -> RegisterSet {
        RegisterSet { regs: HashMap::new() }
    }

    fn clear(&mut self) {
        self.regs.clear();
    }

    fn get(&self, r: char) -> i64 {
        self.regs.get(&r).cloned().unwrap_or(0)
    }

    fn set(&mut self, r: char, v: i64) {
        self.regs.insert(r, v);
    }
}


#[derive(Debug, Clone)]
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


#[derive(Debug, Clone)]
enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value)
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(register<&str, char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
        named!(integer<&str, u64>, map_res!(digit, str::parse));
        named!(number<&str, i64>, alt!(
            preceded!(tag!("-"), integer) => { |n| -(n as i64) } |
                                 integer  => { |n|   n as i64  }
        ));
        named!(value<&str, Value>, alt!(
            register => { |ch| Value::Register(ch) } |
            number   => {  |n| Value::Number(n) }
        ));
        complete!(s, alt!(
            do_parse!(tag!("set") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Set(x, y))) |
            do_parse!(tag!("sub") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Sub(x, y))) |
            do_parse!(tag!("mul") >> x: ws!(register) >> y: ws!(value) >> (Instruction::Mul(x, y))) |
            do_parse!(tag!("jnz") >> x: ws!(value) >> y: ws!(value) >> (Instruction::Jnz(x, y)))
        )).to_result()
    }
}


#[derive(Debug, Clone)]
struct Core {
    code: Vec<Instruction>,
    pc: usize,
    regs: RegisterSet,
    multiplications: usize,
}

impl FromStr for Core {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Core {
            code: try!(s.lines().map(str::parse).collect()),
            pc: 0,
            regs: RegisterSet::new(),
            multiplications: 0,
        })
    }
}

impl Core {
    fn reset(&mut self) {
        self.pc = 0;
        self.regs.clear();
        self.multiplications = 0;
    }

    fn step(&mut self) -> Result<(), ()> {
        match self.code.get(self.pc) {
            Some(ins) => {
                match ins {
                    &Instruction::Set(r, ref v) => {
                        let n = v.get(&self.regs);
                        self.regs.set(r, n)
                    },
                    &Instruction::Sub(r, ref v) => {
                        let n = self.regs.get(r) - v.get(&self.regs);
                        self.regs.set(r, n);
                    },
                    &Instruction::Mul(r, ref v) => {
                        let n = self.regs.get(r) * v.get(&self.regs);
                        self.regs.set(r, n);
                        self.multiplications += 1;
                    },
                    &Instruction::Jnz(ref v, ref ofs) => {
                        if v.get(&self.regs) != 0 {
                            let ofs = ofs.get(&self.regs);
                            self.pc = (self.pc as isize + ofs as isize - 1) as usize;
                        }
                    },
                }
                self.pc += 1;
                Ok(())
            }
            None => Err(()),
        }
    }

    fn run(&mut self) {
        while self.step().is_ok() {}
    }
}


fn main() {
    let mut core: Core = include_str!("day23.txt").parse().unwrap();
    core.run();
    println!("Number of invoked mul instructions: {}", core.multiplications);

    core.reset();
    core.regs.set('a', 1);
    // core.run();
    // println!("Value of register h after completion: {}", core.regs.get('h'));

    // Optimized Rust version:
    //```
    // let h = (0..1000+1).filter(|i| {
    //     let b = 109900 + 17 * i;
    //     (2..b/2).any(|d| (d..b/2).any(|e| d * e == b))
    // }).count();
    // println!("Value of register h after completion: {}", h);
    //```
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert!(Core::from_str(include_str!("day23.txt")).is_ok());
    }
}
