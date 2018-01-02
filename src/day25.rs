use std::collections::HashMap;
use std::hash::Hash;


/// State identifier (a letter)
type StateRef = char;

type ShortTransition<T> = (T, isize, StateRef);
type ShortRule<T> = [(T, ShortTransition<T>)];
type ShortRules<'a, T> = (StateRef, usize, &'a [(StateRef, &'a ShortRule<T>)]);


/// State transition
#[derive(Debug)]
struct Transition<T> {
    /// Value to write
    write_value: T,
    /// Tape cursor offset
    cursor_offset: isize,
    /// Target state
    next_state: StateRef,
}

impl<T> From<ShortTransition<T>> for Transition<T> {
    fn from(other: ShortTransition<T>) -> Self {
        Transition {
            write_value: other.0,
            cursor_offset: other.1,
            next_state: other.2,
        }
    }
}


/// Rule with transitions based on current value
#[derive(Debug)]
struct Rule<T: Eq + Hash> {
    /// Transitions based on current value
    transitions: HashMap<T, Transition<T>>,
}

impl<'a, T: Eq + Hash + Clone> From<&'a ShortRule<T>> for Rule<T> {
    fn from(other: &ShortRule<T>) -> Self {
        Rule {
            transitions: other.iter().cloned().map(|(v, t)| (v, t.into())).collect(),
        }
    }
}

impl<T: Eq + Hash> Rule<T> {
    /// Get transition for the given value
    fn transition(&self, value: &T) -> Option<&Transition<T>> {
        self.transitions.get(value)
    }
}


/// Rules for state transitions of the touring machine
#[derive(Debug)]
struct Rules<T: Eq + Hash> {
    /// Initial state
    initial_state: StateRef,
    /// Number of diagnostic steps to run
    diagnostic_steps: usize,
    /// States with rules for transitions
    rules: HashMap<StateRef, Rule<T>>,
}

impl<'a, T: Eq + Hash + Clone> From<&'a ShortRules<'a, T>> for Rules<T> {
    fn from(other: &ShortRules<T>) -> Self {
        Rules {
            initial_state: other.0,
            diagnostic_steps: other.1,
            rules: other.2.iter().map(|&(rr, r)| (rr, r.into())).collect(),
        }
    }
}

impl<T: Eq + Hash> Rules<T> {
    /// Get transition for the given state and value
    fn transition(&self, state: &StateRef, value: &T) -> Option<&Transition<T>> {
        self.rules.get(state).and_then(|rule| rule.transition(value))
    }
}


/// A tape which contains 0 or 1 infinitely to the left and right
#[derive(Debug)]
struct Tape<T> {
    values: HashMap<isize, T>,
    cursor: isize,
}

impl<T> Tape<T> {
    /// Create a new, blank tape
    fn new() -> Tape<T> {
        Tape { values: HashMap::new(), cursor: 0 }
    }

    /// Move cursor by the given offset
    fn move_cursor(&mut self, offset: isize) {
        self.cursor += offset;
    }
}

impl<T: Default + Eq + Clone> Tape<T> {
    /// Get the value at the cursor position
    fn get_current(&self) -> T {
        self.values.get(&self.cursor).cloned().unwrap_or_default()
    }

    /// Set the value at the cursor position
    fn set_current(&mut self, value: T) {
        self.values.insert(self.cursor, value);
    }
}

impl<T: Default + Eq> Tape<T> {
    /// Calculate checksum (number of nonzero values)
    fn checksum(&self) -> usize {
        self.values.iter().filter(|&(_, v)| v != &T::default()).count()
    }
}


/// Touring machine
#[derive(Debug)]
struct Machine<'a, T: 'a + Eq + Hash> {
    /// Rules for the touring machine
    rules: &'a Rules<T>,
    /// Tape to which the state applies to
    tape: Tape<T>,
    /// Current state
    state: StateRef,
}

impl<'a, T: Default + Eq + Copy + Hash> Machine<'a, T> {
    /// Create new touring machine and do initial diagnosis using the given rules
    fn new(rules: &Rules<T>) -> (Machine<T>, usize) {
        let mut machine = Machine { rules: rules, tape: Tape::new(), state: rules.initial_state };
        if rules.diagnostic_steps > 0 { machine.nth(rules.diagnostic_steps - 1); }
        let checksum = machine.tape.checksum();
        (machine, checksum)
    }
}

impl<'a, T: Default + Eq + Copy + Hash> Iterator for Machine<'a, T> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.rules.transition(&self.state, &self.tape.get_current()).map(|transition| {
            self.tape.set_current(transition.write_value);
            self.tape.move_cursor(transition.cursor_offset);
            self.state = transition.next_state;
        })
    }
}


fn main() {
    let rules: Rules<u8> = (&('A', 12861455, [
        ('A', [(0, (1,  1, 'B')), (1, (0, -1, 'B'))].as_ref()),
        ('B', [(0, (1, -1, 'C')), (1, (0,  1, 'E'))].as_ref()),
        ('C', [(0, (1,  1, 'E')), (1, (0, -1, 'D'))].as_ref()),
        ('D', [(0, (1, -1, 'A')), (1, (1, -1, 'A'))].as_ref()),
        ('E', [(0, (0,  1, 'A')), (1, (0,  1, 'F'))].as_ref()),
        ('F', [(0, (1,  1, 'E')), (1, (1,  1, 'A'))].as_ref()),
    ].as_ref())).into();
    println!("Diagnostic checksum after {} steps: {}", rules.diagnostic_steps, Machine::new(&rules).1);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        let rules: Rules<u8> = (&('A', 6, [
            ('A', [(0, (1,  1, 'B')), (1, (0, -1, 'B'))].as_ref()),
            ('B', [(0, (1, -1, 'A')), (1, (1,  1, 'A'))].as_ref()),
        ].as_ref())).into();
        assert_eq!(Machine::new(&rules).1, 3);
    }
}
