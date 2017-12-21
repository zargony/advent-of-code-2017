#[macro_use]
extern crate nom;

use std::fmt;
use std::str::FromStr;


#[derive(PartialEq, Clone)]
struct Grid {
    pixels: Vec<Vec<bool>>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Grid:"));
        for row in &self.pixels {
            for &pixel in row {
               try!(if pixel { write!(f, "#") } else { write!(f, ".") });
            }
            try!(writeln!(f, ""));
        }
        Ok(())
    }
}

impl Grid {
    /// Create new Grid with initial pixels
    fn new() -> Grid {
        Grid {
            pixels: vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]],
        }
    }

    /// Size of grid (edge length)
    fn size(&self) -> usize {
        self.pixels.len()
    }

    /// Number of lit pixels
    fn lit_pixels(&self) -> usize {
        self.pixels.iter().map(|r| r.iter().filter(|&&p| p).count()).sum()
    }

    /// Returns the subgrid of the given size and position
    fn subgrid(&self, row: usize, col: usize, size: usize) -> Grid {
        Grid {
            pixels: (0..size).map(|r|
                self.pixels[row+r][col..col+size].to_owned()
            ).collect()
        }
    }

    /// Partition the grid into subgrids with edge size 2 or 3
    fn subgrids(&self) -> Vec<Grid> {
        let subsize = if self.size() % 2 == 0 { 2 }
            else if self.size() % 3 == 0 { 3 }
            else { panic!("Couldn't partition grid of size {}", self.size()) };
        let n = self.size() / subsize;
        let mut grids = vec![];
        for r in 0..n {
            for c in 0..n {
                grids.push(self.subgrid(r*subsize, c*subsize, subsize));
            }
        }
        grids
    }

    /// Append grid to the right (must be of same height)
    fn append_right(&mut self, other: &Grid) {
        assert_eq!(other.pixels.len(), self.pixels.len());
        for i in 0..self.pixels.len() {
            self.pixels[i].extend(&other.pixels[i]);
        }
    }

    /// Append grid to the bottom (must be of same width)
    fn append_bottom(&mut self, other: &Grid) {
        assert_eq!(other.pixels[0].len(), self.pixels[0].len());
        for r in &other.pixels {
            self.pixels.push(r.clone());
        }
    }

    /// Build grid from n*n given subgrids
    fn build(grids: &[Grid]) -> Grid {
        let mut grid = grids[0].clone();
        let n = (grids.len() as f32).sqrt() as usize;
        for j in 1..n {
            grid.append_right(&grids[j]);
        }
        for i in 1..n {
            let mut row = grids[i*n].clone();
            for j in 1..n {
                row.append_right(&grids[i*n+j]);
            }
            grid.append_bottom(&row);
        }
        grid
    }

    /// Returns the grid rotated by 90° ccw
    fn rotate(&self) -> Grid {
        Grid {
            pixels: (0..self.size()).map(|r|
                (0..self.size()).map(|c|
                    self.pixels[c][self.size()-r-1]
                ).collect()
            ).collect()
        }
    }

    /// Returns the grid mirrored vertically
    fn mirror(&self) -> Grid {
        Grid {
            pixels: self.pixels.iter().map(|row|
                row.iter().rev().cloned().collect()
            ).collect()
        }
    }

    /// Check if the grid matches the given other grid (in any orientation)
    fn matches(&self, other: &Grid) -> bool {
        if self.size() != other.size() { return false; }
        if self == other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        let other = other.rotate().mirror();
        if self == &other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        let other = other.rotate();
        if self == &other { return true; }
        false
    }
}


#[derive(Debug)]
struct Rule {
    search: Grid,
    replace: Grid,
}

impl FromStr for Rule {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(pixel<&str, bool>, alt!(tag!(".") => { |_| false } | tag!("#") => { |_| true }));
        named!(row<&str, Vec<bool>>, many1!(pixel));
        named!(grid<&str, Grid>, map!(separated_list_complete!(tag!("/"), row), |v| Grid { pixels: v }));
        complete!(s, do_parse!(
            search: ws!(grid) >> tag!("=>") >> replace: ws!(grid) >>
            (Rule { search: search, replace: replace })
        )).to_result()
    }
}


#[derive(Debug)]
struct Book(Vec<Rule>);

impl FromStr for Book {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Book(try!(s.lines().map(str::parse).collect())))
    }
}

impl Book {
    /// Find the replacement grid for the given grid
    fn matches(&self, grid: &Grid) -> Option<Grid> {
        self.0.iter().find(|rule|
            grid.matches(&rule.search)
        ).map(|rule|
            rule.replace.clone()
        )
    }

    /// Apply rules on all subgrids of the given grid
    fn apply(&self, grid: &Grid) -> Option<Grid> {
        grid.subgrids().iter().map(|g|
            self.matches(g)
        ).collect::<Option<Vec<Grid>>>().map(|g|
            Grid::build(&g)
        )
    }
}


fn main() {
    let book: Book = include_str!("day21.txt").parse().unwrap();
    let mut grid = Grid::new();
    for _ in 0..5 { grid = book.apply(&grid).unwrap(); }
    println!("Lit pixels after 5 iterations: {}", grid.lit_pixels());
    for _ in 5..18 { grid = book.apply(&grid).unwrap(); }
    println!("Lit pixels after 18 iterations: {}", grid.lit_pixels());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let _rule = Rule::from_str("../.# => ##./#../...").unwrap();
        let _book = Book::from_str("../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#\n").unwrap();
    }

    #[test]
    fn divide_and_merge() {
        let grids: Vec<Grid> = (0..4).map(|_| Grid::new()).collect();
        assert_eq!(grids[0].pixels, vec![
            vec![false,  true, false],
            vec![false, false,  true],
            vec![ true,  true,  true]
        ]);
        let grid = Grid::build(&grids);
        assert_eq!(grid.pixels, vec![
            vec![false,  true, false, false,  true, false],
            vec![false, false,  true, false, false,  true],
            vec![ true,  true,  true,  true,  true,  true],
            vec![false,  true, false, false,  true, false],
            vec![false, false,  true, false, false,  true],
            vec![ true,  true,  true,  true,  true,  true],
        ]);
    }

    #[test]
    fn rotation() {
        let grid = Grid::new();
        assert_eq!(grid.pixels, vec![
            vec![false,  true, false],
            vec![false, false,  true],
            vec![ true,  true,  true]
        ]);
        let grid = grid.rotate();
        assert_eq!(grid.pixels, vec![
            vec![false,  true,  true],
            vec![ true, false,  true],
            vec![false, false,  true]
        ]);
        let grid = grid.rotate();
        assert_eq!(grid.pixels, vec![
            vec![ true,  true,  true],
            vec![ true, false, false],
            vec![false,  true, false]
        ]);
        let grid = grid.rotate();
        assert_eq!(grid.pixels, vec![
            vec![ true, false, false],
            vec![ true, false,  true],
            vec![ true,  true, false]
        ]);
    }

    #[test]
    fn mirroring() {
        let grid = Grid::new();
        assert_eq!(grid.pixels, vec![
            vec![false,  true, false],
            vec![false, false,  true],
            vec![ true,  true,  true]
        ]);
        let grid = grid.mirror();
        assert_eq!(grid.pixels, vec![
            vec![false,  true, false],
            vec![ true, false, false],
            vec![ true,  true,  true]
        ]);
    }

    #[test]
    fn samples() {
        let book = Book::from_str("../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#\n").unwrap();
        let grid = Grid::new();
        assert_eq!(grid.size(), 3);
        assert_eq!(grid.lit_pixels(), 5);
        let grid = book.apply(&grid).unwrap();
        assert_eq!(grid.size(), 4);
        assert_eq!(grid.lit_pixels(), 4);
    }
}
