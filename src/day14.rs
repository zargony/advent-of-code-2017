#[allow(dead_code)]
mod day10;


/// A disk usage map tracking free and used blocks
struct DiskUsage {
    grid: [[bool; 128]; 128],
}

impl DiskUsage {
    /// Create new disk usage state from given key using knot hashing
    fn new(key: &str) -> DiskUsage {
        let mut grid = [[false; 128]; 128];
        for y in 0..128 {
            let mut hasher = day10::KnotHasher::new();
            hasher.write(&format!("{}-{}", key, y));
            let hash = hasher.finish();
            for x in 0..128 {
                grid[y][x] = hash[x / 8] & 0x80 >> (x % 8) > 0;
            }
        }
        DiskUsage { grid: grid }
    }

    /// Returns the number of used blocks
    fn used(&self) -> usize {
        self.grid.iter().map(|row| row.iter().filter(|b| **b).count()).sum()
    }

    /// Returns the number of separate regions
    fn regions(mut self) -> usize {
        let mut count = 0;
        for y in 0..128 {
            for x in 0..128 {
                if self.grid[y][x] {
                    self.clear_region(y, x);
                    count += 1;
                }
            }
        }
        count
    }

    /// Clear all blocks of a region starting at the given block position
    fn clear_region(&mut self, y: usize, x: usize) {
        if self.grid[y][x] {
            self.grid[y][x] = false;
            if x >   0 { self.clear_region(y, x-1); }
            if x < 127 { self.clear_region(y, x+1); }
            if y >   0 { self.clear_region(y-1, x); }
            if y < 127 { self.clear_region(y+1, x); }
        }
    }
}


fn main() {
    const INPUT: &str = "hfdlxzhv";
    let disk = DiskUsage::new(INPUT);
    println!("Used squares: {}", disk.used());
    println!("Known regions: {}", disk.regions());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating() {
        let disk = DiskUsage::new("flqrgnkx");
        assert_eq!(disk.grid[0][0..8], [ true,  true, false,  true, false,  true, false, false]);
        assert_eq!(disk.grid[1][0..8], [false,  true, false,  true, false,  true, false,  true]);
        assert_eq!(disk.grid[2][0..8], [false, false, false, false,  true, false,  true, false]);
        assert_eq!(disk.grid[3][0..8], [ true, false,  true, false,  true,  true, false,  true]);
        assert_eq!(disk.grid[4][0..8], [false,  true,  true, false,  true, false, false, false]);
        assert_eq!(disk.grid[5][0..8], [ true,  true, false, false,  true, false, false,  true]);
        assert_eq!(disk.grid[6][0..8], [false,  true, false, false, false,  true, false, false]);
        assert_eq!(disk.grid[7][0..8], [ true,  true, false,  true, false,  true,  true, false]);
    }

    #[test]
    fn samples1() {
        let disk = DiskUsage::new("flqrgnkx");
        assert_eq!(disk.used(), 8108);
    }

    #[test]
    fn samples2() {
        let disk = DiskUsage::new("flqrgnkx");
        assert_eq!(disk.regions(), 1242);
    }
}
