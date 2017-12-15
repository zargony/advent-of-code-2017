use std::fmt;


/// Knot Hasher using a Knot Hash Ring
#[derive(Debug)]
pub struct KnotHasher {
    /// Elements of the ring
    elements: Vec<u8>,
    /// Current position
    position: usize,
    /// Current skip size
    skip: usize,
}

impl fmt::LowerHex for KnotHasher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.finish() {
            try!(write!(f, "{:02x}", block));
        }
        Ok(())
    }
}

impl KnotHasher {
    /// Create a new Ring
    pub fn new() -> KnotHasher {
        KnotHasher { elements: (0..256).map(|b| b as u8).collect(), position: 0, skip: 0 }
    }

    /// Reverse the given length of elements at the current position
    fn reverse(&mut self, step: usize) {
        let len = self.elements.len();
        for i in 0 .. step / 2 {
            self.elements.swap((self.position + i) % len, (self.position + step - i - 1) % len);
        }
        self.position = (self.position + step + self.skip) % len;
        self.skip += 1;
    }

    /// Do 64 hash rounds using the given byte sequence
    pub fn write<T: AsRef<[u8]>>(&mut self, bytes: T) {
        for _ in 0..64 {
            for b in bytes.as_ref() {
                self.reverse(*b as usize);
            }
            for b in &[17, 31, 73, 47, 23] {
                self.reverse(*b);
            }
        }
    }

    /// Resulting hash value
    pub fn finish(&self) -> [u8; 16] {
        self.elements.chunks(16).enumerate().fold([0; 16], |mut hash, (i, block)| {
            hash[i] = block.iter().fold(0, |h, b| h ^ b);
            hash
        })
    }
}


fn main() {
    const INPUT: &str = "70,66,255,2,48,0,54,48,80,141,244,254,160,108,1,41";

    let mut ring = KnotHasher::new();
    for step in INPUT.split(',').map(str::parse) {
        ring.reverse(step.unwrap())
    }
    println!("Resulting value of first test round: {}", ring.elements[0] as u32 * ring.elements[1] as u32);

    let mut ring = KnotHasher::new();
    ring.write(INPUT);
    println!("Resulting knot hash: {:x}", ring);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let mut ring = KnotHasher::new();
        ring.elements = (0..5).collect();
        assert_eq!(ring.elements, vec![0, 1, 2, 3, 4]);
        ring.reverse(3);
        assert_eq!(ring.elements, vec![2, 1, 0, 3, 4]);
        ring.reverse(4);
        assert_eq!(ring.elements, vec![4, 3, 0, 1, 2]);
        ring.reverse(1);
        assert_eq!(ring.elements, vec![4, 3, 0, 1, 2]);
        ring.reverse(5);
        assert_eq!(ring.elements, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn samples2() {
        let mut ring = KnotHasher::new();
        ring.write("");
        assert_eq!(format!("{:x}", ring), "a2582a3a0e66e6e86e3812dcb672a272");
        let mut ring = KnotHasher::new();
        ring.write("AoC 2017");
        assert_eq!(format!("{:x}", ring), "33efeb34ea91902bb2f59c9920caa6cd");
        let mut ring = KnotHasher::new();
        ring.write("1,2,3");
        assert_eq!(format!("{:x}", ring), "3efbe78a8d82f29979031a4aa0b16a9d");
        let mut ring = KnotHasher::new();
        ring.write("1,2,4");
        assert_eq!(format!("{:x}", ring), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
