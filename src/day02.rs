use std::str::FromStr;


/// The spreadsheet
#[derive(Debug, PartialEq)]
struct Spreadsheet {
    /// Vector of rows with vector of numbers
    values: Vec<Vec<u32>>,
}

impl FromStr for Spreadsheet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Spreadsheet {
            values: s.lines().map(|line| {
                line.split_whitespace().map(|word| {
                    word.parse().expect("Invalid number")
                }).collect()
            }).collect()
        })
    }
}

impl Spreadsheet {
    /// Checksum of spreadsheet (sum of differences of largest and smalles values of each row)
    fn checksum(&self) -> u32 {
        self.values.iter().map(|row| {
            row.iter().max().unwrap() - row.iter().min().unwrap()
        }).sum()
    }

    /// Divsum of spreadsheet (sum of the two evenly divisable values of each row)
    fn divsum(&self) -> u32 {
        self.values.iter().map(|row| {
            for a in row.iter() {
                for b in row.iter() {
                    if a != b && a % b == 0 { return a / b }
                }
            }
            unreachable!()
        }).sum()
    }
}


fn main() {
    let spreadsheet: Spreadsheet = include_str!("day02.txt").parse().unwrap();
    println!("Checksum of spreadsheet: {}", spreadsheet.checksum());
    println!("Divsum of spreadsheet: {}", spreadsheet.divsum());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Spreadsheet::from_str("5 1 9 5\n7 5 3\n2 4 6 8"), Ok(Spreadsheet { values: vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]] }));
    }

    #[test]
    fn samples1() {
        assert_eq!(Spreadsheet::from_str("5 1 9 5\n7 5 3\n2 4 6 8").unwrap().checksum(), 18);
    }

    #[test]
    fn samples2() {
        assert_eq!(Spreadsheet::from_str("5 9 2 8\n9 4 7 3\n3 8 6 5").unwrap().divsum(), 9);
    }
}
