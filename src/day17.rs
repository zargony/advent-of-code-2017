/// Create a spinlock's ring buffer and return the value to
/// short-circuit (value after last inserted value)
fn spinlock_short_circuit(stepsize: usize, iterations: usize) -> u32 {
    let mut buffer: Vec<u32> = vec![0];
    let mut position = 0;
    for i in 1..iterations as u32 + 1 {
        position = (position + stepsize) % buffer.len();
        if position == buffer.len()-1 {
            buffer.push(i);
        } else {
            buffer.insert(position+1, i);
        }
        position += 1;
    }
    buffer[(position + 1) % buffer.len()]
}

/// Return the improved value to short-circuit the spinlock
/// (value after zero, i.e. the second value) without actually
/// building the whole spinlock
fn spinlock_short_circuit_improved(stepsize: usize, iterations: usize) -> u32 {
    let mut value = 0;
    let mut position = 0;
    for i in 1..iterations as u32 + 1 {
        position = (position + stepsize) % i as usize;
        if position == 0 { value = i; }
        position += 1;
    }
    value
}


fn main() {
    const INPUT: usize = 371;
    println!("Spinlock shortcut value: {}", spinlock_short_circuit(INPUT, 2017));
    println!("Spinlock improved shortcut value: {}", spinlock_short_circuit_improved(INPUT, 50_000_000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1() {
        assert_eq!(spinlock_short_circuit(3, 2017), 638);
    }

    #[test]
    fn samples2() {
        assert_eq!(spinlock_short_circuit_improved(3, 2017), 1226);
    }
}
