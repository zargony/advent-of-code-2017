#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::str::FromStr;
use nom::{alpha, digit};


/// Operation that can be executed on a value
#[derive(Debug, PartialEq)]
enum Operation {
    Inc(i32), Dec(i32)
}

impl Operation {
    /// Execute operation on the given value
    fn execute(&self, value: i32) -> i32 {
        match *self {
            Operation::Inc(operand) => value + operand,
            Operation::Dec(operand) => value - operand,
        }
    }
}


/// Condition that can be queried
#[derive(Debug, PartialEq)]
enum Condition {
    Eq(i32), Ne(i32), Lt(i32), Le(i32), Gt(i32), Ge(i32)
}

impl Condition {
    /// Check condition on the given value
    fn check(&self, value: i32) -> bool {
        match *self {
            Condition::Eq(operand) => value == operand,
            Condition::Ne(operand) => value != operand,
            Condition::Lt(operand) => value < operand,
            Condition::Le(operand) => value <= operand,
            Condition::Gt(operand) => value > operand,
            Condition::Ge(operand) => value >= operand,
        }
    }
}


/// A single instruction
#[derive(Debug, PartialEq)]
struct Instruction {
    target_register: String,
    operation: Operation,
    check_register: String,
    condition: Condition,
}

impl FromStr for Instruction {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(identifier<&str, String>, map_res!(ws!(alpha), str::parse));
        named!(number<&str, i32>, map_res!(ws!(digit), str::parse));
        named!(value<&str, i32>, alt!(
            map!(preceded!(tag!("-"), number), |x| -x) |
            number
        ));
        named!(operation<&str, Operation>, alt!(
            map!(preceded!(tag!("inc"), ws!(value)), |x| Operation::Inc(x)) |
            map!(preceded!(tag!("dec"), ws!(value)), |x| Operation::Dec(x))
        ));
        named!(condition<&str, Condition>, alt!(
            map!(preceded!(tag!("=="), ws!(value)), |x| Condition::Eq(x)) |
            map!(preceded!(tag!("!="), ws!(value)), |x| Condition::Ne(x)) |
            map!(preceded!(tag!("<"), ws!(value)), |x| Condition::Lt(x)) |
            map!(preceded!(tag!("<="), ws!(value)), |x| Condition::Le(x)) |
            map!(preceded!(tag!(">"), ws!(value)), |x| Condition::Gt(x)) |
            map!(preceded!(tag!(">="), ws!(value)), |x| Condition::Ge(x))
        ));
        complete!(s, do_parse!(
            target_register: identifier >>
            operation: operation >>
            tag!("if") >>
            check_register: identifier >>
            condition: condition >>
            (Instruction { target_register: target_register, operation: operation, check_register: check_register, condition: condition })
        )).to_result()
    }
}


/// A series of instructions to execute
#[derive(Debug)]
struct Code {
    instructions: Vec<Instruction>,
}

impl FromStr for Code {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Code { instructions: s.lines().map(|l| l.parse().unwrap()).collect() })
    }
}

impl Code {
    /// Run all instructions and return final state
    fn run(&self) -> State {
        let mut state = State::new(self);
        state.run();
        state
    }
}


/// Current state of executing code
#[derive(Debug)]
struct State<'a> {
    code: &'a Code,
    current: usize,
    registers: HashMap<String, i32>,
    highest_value: Option<i32>,
}

impl<'a> State<'a> {
    /// Create new state for the given code
    fn new(code: &Code) -> State {
        State { code: code, current: 0, registers: HashMap::new(), highest_value: None }
    }

    /// Run one instruction
    fn step(&mut self) -> bool {
        if self.current < self.code.instructions.len() {
            let ins = &self.code.instructions[self.current];
            let reg = *self.registers.get(&ins.check_register).unwrap_or(&0);
            if ins.condition.check(reg) {
                let reg = self.registers.entry(ins.target_register.clone()).or_insert(0);
                *reg = ins.operation.execute(*reg);
                self.highest_value = std::cmp::max(self.highest_value, Some(*reg));
            }
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Run all instructions
    fn run(&mut self) {
        while self.step() {}
    }

    /// Returns the largest value in any register of the current state
    fn largest_value(&self) -> Option<i32> {
        self.registers.iter().map(|(_, &value)| value).max()
    }

    /// Returns the largest value in any register of any previous state
    fn largest_value_ever(&self) -> Option<i32> {
        self.highest_value
    }
}


fn main() {
    let code: Code = include_str!("day08.txt").parse().unwrap();
    let state = code.run();
    println!("Largest value in any register after execution: {}", state.largest_value().unwrap());
    println!("Highest value ever seen in any register: {}", state.largest_value_ever().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Instruction::from_str("b inc 5 if a > 1"), Ok(Instruction { target_register: "b".to_string(), operation: Operation::Inc(5), check_register: "a".to_string(), condition: Condition::Gt(1) }));
        assert_eq!(Instruction::from_str("a inc 1 if b < 5"), Ok(Instruction { target_register: "a".to_string(), operation: Operation::Inc(1), check_register: "b".to_string(), condition: Condition::Lt(5) }));
        assert_eq!(Instruction::from_str("c dec -10 if a >= 1"), Ok(Instruction { target_register: "c".to_string(), operation: Operation::Dec(-10), check_register: "a".to_string(), condition: Condition::Ge(1) }));
        assert_eq!(Instruction::from_str("c inc -20 if c == 10"), Ok(Instruction { target_register: "c".to_string(), operation: Operation::Inc(-20), check_register: "c".to_string(), condition: Condition::Eq(10) }));
    }

    #[test]
    fn samples() {
        let code: Code = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10".parse().unwrap();
        let state = code.run();
        assert_eq!(state.largest_value(), Some(1));
        assert_eq!(state.largest_value_ever(), Some(10));
    }
}
