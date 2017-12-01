use std::str::FromStr;


/// The captcha to solve
#[derive(Debug, PartialEq)]
struct Captcha {
    digits: Vec<u32>,
}

impl FromStr for Captcha {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Captcha {
            digits: s.chars().map(|ch| {
                ch.to_digit(10).expect("Invalid digit")
            }).collect()
        })
    }
}

impl Captcha {
    /// Returns the sum of all digits that matches its nth successor
    fn sumx(&self, n: usize) -> u32 {
        let len = self.digits.len();
        self.digits.iter().enumerate().fold(0, |res, (i, x)| {
            res + if *x == self.digits[(i + n) % len] { *x } else { 0 }
        })
    }

    /// Returns the sum of all digits that matches its immediate successor
    fn sum(&self) -> u32 {
        self.sumx(1)
    }

    /// Returns the sum of all digits that matches the opposite digit
    fn midsum(&self) -> u32 {
        self.sumx(self.digits.len() / 2)
    }
}


fn main() {
    let captcha: Captcha = include_str!("day01.txt").parse().unwrap();
    println!("Sum (next) of captcha: {}", captcha.sum());
    println!("Sum (mid) of captcha: {}", captcha.midsum());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Captcha::from_str("1234"), Ok(Captcha { digits: vec![1, 2, 3, 4] }));
    }

    #[test]
    fn samples1() {
        assert_eq!(Captcha::from_str("1122").unwrap().sum(), 3);
        assert_eq!(Captcha::from_str("1111").unwrap().sum(), 4);
        assert_eq!(Captcha::from_str("1234").unwrap().sum(), 0);
        assert_eq!(Captcha::from_str("91212129").unwrap().sum(), 9);
    }

    #[test]
    fn samples2() {
        assert_eq!(Captcha::from_str("1212").unwrap().midsum(), 6);
        assert_eq!(Captcha::from_str("1221").unwrap().midsum(), 0);
        assert_eq!(Captcha::from_str("123425").unwrap().midsum(), 4);
        assert_eq!(Captcha::from_str("123123").unwrap().midsum(), 12);
        assert_eq!(Captcha::from_str("12131415").unwrap().midsum(), 4);
    }
}
