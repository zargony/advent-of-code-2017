use std::collections::HashSet;
use std::str::FromStr;


/// A passphrase
#[derive(Debug, PartialEq)]
struct Passphrase {
    /// Vector of words
    words: Vec<String>,
}

impl FromStr for Passphrase {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Passphrase { words: s.split_whitespace().map(|s| s.to_string()).collect() })
    }
}

impl Passphrase {
    /// Check if passphrase is valid (contains no repeating words)
    fn is_valid(&self) -> bool {
        let mut check = HashSet::new();
        for word in &self.words {
            if check.contains(word) { return false; }
            check.insert(word);
        }
        true
    }

    /// Check if passphrase is valid (contains no repeating anagrams)
    fn is_valid2(&self) -> bool {
        let mut check = HashSet::new();
        for word in &self.words {
            let mut key: Vec<char> = word.chars().collect();
            key.sort();
            if check.contains(&key) { return false; }
            check.insert(key);
        }
        true
    }
}


fn main() {
    let passphrases: Vec<Passphrase> = include_str!("day04.txt").lines().map(|l| l.parse().unwrap()).collect();
    println!("Number of valid passphrases: {}", passphrases.iter().filter(|p| p.is_valid()).count());
    println!("Number of new valid passphrases: {}", passphrases.iter().filter(|p| p.is_valid2()).count());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1() {
        assert!(Passphrase::from_str("aa bb cc dd ee").unwrap().is_valid());
        assert!(!Passphrase::from_str("aa bb cc dd aa").unwrap().is_valid());
        assert!(Passphrase::from_str("aa bb cc dd aaa").unwrap().is_valid());
    }

    #[test]
    fn samples2() {
        assert!(Passphrase::from_str("abcde fghij").unwrap().is_valid2());
        assert!(!Passphrase::from_str("abcde xyz ecdab").unwrap().is_valid2());
        assert!(Passphrase::from_str("a ab abc abd abf abj").unwrap().is_valid2());
        assert!(Passphrase::from_str("iiii oiii ooii oooi oooo").unwrap().is_valid2());
        assert!(!Passphrase::from_str("oiii ioii iioi iiio").unwrap().is_valid2());
    }
}
