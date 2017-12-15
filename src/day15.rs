/// Number generator
#[derive(Debug)]
struct Generator {
    factor: u32,
    value: u32,
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = ((self.value as u64 * self.factor as u64) % 2147483647) as u32;
        Some(self.value)
    }
}

impl Generator {
    /// Create new number generator with the given factor and starting value
    fn new(factor: u32, value: u32) -> Generator {
        Generator { factor: factor, value: value }
    }
}


/// Compare next n outputs of the given two generators. Returns the number of
/// outputs where the least 16 bit are matching
fn compare_generators<I, J>(a: &mut I, b: &mut J, n: u32) -> usize
    where I: Iterator<Item=u32>,
          J: Iterator<Item=u32>,
{
    (0..n).map(|_|
        (a.next().unwrap(), b.next().unwrap())
    ).filter(|&(a, b)|
        a & 0xffff == b & 0xffff
    ).count()
}


fn main() {
    const INPUT: (u32, u32) = (634, 301);
    let mut generator_a = Generator::new(16807, INPUT.0);
    let mut generator_b = Generator::new(48271, INPUT.1);
    println!("Final count after 40 million pairs: {}", compare_generators(&mut generator_a, &mut generator_b, 40_000_000));
    let mut generator_a = Generator::new(16807, INPUT.0).filter(|v| v % 4 == 0);
    let mut generator_b = Generator::new(48271, INPUT.1).filter(|v| v % 8 == 0);
    println!("Final count after 5 million pairs: {}", compare_generators(&mut generator_a, &mut generator_b, 5_000_000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1a() {
        let mut generator_a = Generator::new(16807, 65);
        let mut generator_b = Generator::new(48271, 8921);
        assert_eq!(generator_a.next(), Some(1092455));
        assert_eq!(generator_b.next(), Some(430625591));
        assert_eq!(generator_a.next(), Some(1181022009));
        assert_eq!(generator_b.next(), Some(1233683848));
        assert_eq!(generator_a.next(), Some(245556042));
        assert_eq!(generator_b.next(), Some(1431495498));
        assert_eq!(generator_a.next(), Some(1744312007));
        assert_eq!(generator_b.next(), Some(137874439));
        assert_eq!(generator_a.next(), Some(1352636452));
        assert_eq!(generator_b.next(), Some(285222916));
    }

    #[test]
    fn samples1b() {
        let mut generator_a = Generator::new(16807, 65);
        let mut generator_b = Generator::new(48271, 8921);
        assert_eq!(compare_generators(&mut generator_a, &mut generator_b, 40_000_000), 588);
    }

    #[test]
    fn samples2a() {
        let mut generator_a = Generator::new(16807, 65).filter(|v| v % 4 == 0);
        let mut generator_b = Generator::new(48271, 8921).filter(|v| v % 8 == 0);
        assert_eq!(generator_a.next(), Some(1352636452));
        assert_eq!(generator_b.next(), Some(1233683848));
        assert_eq!(generator_a.next(), Some(1992081072));
        assert_eq!(generator_b.next(), Some(862516352));
        assert_eq!(generator_a.next(), Some(530830436));
        assert_eq!(generator_b.next(), Some(1159784568));
        assert_eq!(generator_a.next(), Some(1980017072));
        assert_eq!(generator_b.next(), Some(1616057672));
        assert_eq!(generator_a.next(), Some(740335192));
        assert_eq!(generator_b.next(), Some(412269392));
    }

    #[test]
    fn samples2b() {
        let mut generator_a = Generator::new(16807, 65).filter(|v| v % 4 == 0);
        let mut generator_b = Generator::new(48271, 8921).filter(|v| v % 8 == 0);
        assert_eq!(compare_generators(&mut generator_a, &mut generator_b, 5_000_000), 309);
    }
}
