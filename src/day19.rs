use std::str::FromStr;


/// The world. Consists of a two-dimensional landscape of fields with only some of them being walkable.
#[derive(Debug)]
struct World {
    /// A two-dimensional landscape of fields in the world. A field may either exist (being walkable)
    /// or not. If it exists, it may optionally contain a letter.
    fields: Vec<Vec<Option<Option<char>>>>,
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(World {
            fields: s.lines().map(|line|
                line.chars().map(|ch| match ch {
                    'A'...'Z' => Some(Some(ch)),
                    ' '       => None,
                    _         => Some(None),
                }).collect()
            ).collect(),
        })
    }
}

impl World {
    /// Returns the field and its optional letter at the given row and column
    fn field(&self, row: usize, col: usize) -> Option<Option<char>> {
        self.fields.get(row).and_then(|r| r.get(col)).and_then(|f| *f)
    }

    /// Returns an iterator that can be used to walk the path
    fn path(&self) -> Path {
        let start_col = self.fields[0].iter().position(Option::is_some).expect("Begin of path not found");
        Path { world: self, row: 0, col: start_col, dir: Direction::South }
    }
}


/// Cardinal direction
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North, East, South, West,
}

impl Direction {
    /// Returns the new direction when turning left
    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East  => Direction::North,
            Direction::South => Direction::East,
            Direction::West  => Direction::South,
        }
    }

    /// Returns the new direction when turning right
    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
        }
    }
}


/// Path iterator for walking through the world
#[derive(Debug)]
struct Path<'a> {
    world: &'a World,
    row: usize,
    col: usize,
    dir: Direction,
}

impl<'a> Iterator for Path<'a> {
    type Item = (usize, usize, Option<char>);

    fn next(&mut self) -> Option<Self::Item> {
        fn try_field(world: &World, row: usize, col: usize) -> Option<(usize, usize, Option<char>)> {
            world.field(row, col).map(|f| (row, col, f))
        }
        fn try_walk(world: &World, row: usize, col: usize, dir: Direction) -> Option<(usize, usize, Option<char>)> {
            match dir {
                Direction::North if row > 0 => try_field(world, row - 1, col    ),
                Direction::East             => try_field(world, row,     col + 1),
                Direction::South            => try_field(world, row + 1, col    ),
                Direction::West  if col > 0 => try_field(world, row,     col - 1),
                _                            => None,
            }
        }
        for &dir in &[self.dir, self.dir.turn_left(), self.dir.turn_right()] {
            if let Some((row, col, ch)) = try_walk(&self.world, self.row, self.col, dir) {
                self.row = row;
                self.col = col;
                self.dir = dir;
                return Some((row, col, ch));
            }
        }
        None
    }
}

impl<'a> Path<'a> {
    /// Consumes the path iterator and returns a letter iterator that yields
    /// the letters on the path
    fn letters(self) -> Letters<'a> {
        Letters { path: self }
    }
}


/// Letter iterator for collecting letters on a walked path
#[derive(Debug)]
struct Letters<'a> {
    path: Path<'a>,
}

impl<'a> Iterator for Letters<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.path.next() {
            Some((_, _, Some(ch))) => Some(ch),
            Some(_) => self.next(),
            None => None,
        }
    }
}


fn main() {
    let world: World = include_str!("day19.txt").parse().unwrap();
    println!("Letters seen on path: {}", world.path().letters().collect::<String>());
    println!("Steps needed to go: {}", world.path().count() + 1);
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "     |          \n     |  +--+    \n     A  |  C    \n F---|----E|--+ \n     |  |  |  D \n     +B-+  +--+ \n\n";

    #[test]
    fn samples() {
        let world = World::from_str(INPUT).unwrap();
        assert_eq!(world.path().letters().collect::<String>(), "ABCDEF");
        assert_eq!(world.path().count() + 1, 38);
    }
}
